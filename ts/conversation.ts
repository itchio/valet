import {
  NotificationHandler,
  RequestHandler,
  createResult,
  RequestCreator,
  NotificationCreator,
  StandardErrorCode,
  RpcError,
  ResultCreator,
  RequestError,
  RpcMessage,
  InternalCode,
} from "./support";
import valet, { Conn } from ".";
import { Client } from "./client";

interface RequestHandlers {
  [method: string]: RequestHandler<any, any>;
}

interface NotificationHandlers {
  [method: string]: NotificationHandler<any>;
}

interface OutboundRequest {
  resolve: (payload: any) => void;
  reject: (err: Error) => void;
}

const genericResult = createResult<any>();

export class Conversation {
  private cancelled: boolean = false;
  private conn: Conn;
  private closed: boolean = false;
  private notificationHandlers: NotificationHandlers = {};
  private missingNotificationHandlersWarned: { [key: string]: boolean } = {};
  private requestHandlers: RequestHandlers = {};
  private client: Client;
  private inboundRequests: {
    [key: number]: boolean;
  } = {};
  private outboundRequests: {
    [key: number]: OutboundRequest;
  } = {};
  private firstMethod?: string;

  constructor(client: Client) {
    this.client = client;
    this.conn = valet.newConn();
    this.run().catch((e) => {
      this.client.warn(`While processing incoming messages: ${e.stack}`);
    });
  }

  private async run() {
    while (true) {
      let msg = await this.conn.recv();
      try {
        await this.handleMessage(JSON.parse(msg) as RpcMessage);
      } catch (e) {
        this.client.warn(`While processing message: ${e.stack}`);
      }
    }
  }

  onRequest<Params, Result>(
    rc: RequestCreator<Params, Result>,
    handler: RequestHandler<Params, Result>
  ) {
    if (this.requestHandlers[rc.__method]) {
      throw new Error(
        `cannot register a second request handler for ${rc.__method}`
      );
    }
    this.requestHandlers[rc.__method] = handler;
  }

  onNotification<T>(
    nc: NotificationCreator<T>,
    handler: NotificationHandler<T>
  ) {
    if (this.notificationHandlers[nc.__method]) {
      throw new Error(
        `cannot register a second notification handler for ${nc.__method}`
      );
    }
    this.notificationHandlers[nc.__method] = handler;
  }

  private async handleMessage(msg: RpcMessage) {
    if (this.cancelled) {
      return;
    }

    if (typeof msg !== "object") {
      return;
    }

    if (msg.jsonrpc != "2.0") {
      return;
    }

    if (typeof msg.id === "undefined") {
      // we got a notification!
      const handler = this.notificationHandlers[msg.method];
      if (!handler) {
        if (!this.missingNotificationHandlersWarned[msg.method]) {
          this.missingNotificationHandlersWarned[msg.method] = true;
          this.client.warn(
            `no handler for notification ${msg.method} (in ${this.firstMethod} convo)`
          );
        }
        return;
      }

      try {
        await Promise.resolve(handler(msg.params));
      } catch (e) {
        this.client.warn(`notification handler error: ${e.stack}`);
        if (this.client.errorHandler) {
          this.client.errorHandler(e);
        }
      }

      return;
    }

    if (msg.method) {
      try {
        this.inboundRequests[msg.id] = true;

        let receivedAt = Date.now();
        const handler = this.requestHandlers[msg.method];
        if (!handler) {
          if (this.cancelled) {
            return;
          }
          this.sendResult(genericResult, msg.id, null, <RpcError>{
            code: StandardErrorCode.MethodNotFound,
            message: `no handler is registered for method ${msg.method}`,
          });
          return;
        }

        try {
          const result = await handler(msg.params);
          if (this.cancelled) {
            return;
          }
          this.sendResult(genericResult, msg.id, result, undefined);
        } catch (e) {
          if (this.cancelled) {
            return;
          }
          this.sendResult(genericResult, msg.id, null, <RpcError>{
            code: StandardErrorCode.InternalError,
            message: `async error: ${e.message}`,
            data: {
              stack: e.stack,
            },
          });
        }
      } finally {
        delete this.inboundRequests[msg.id];
      }
      return;
    }

    if (msg.result || msg.error) {
      let req = this.outboundRequests[msg.id];
      delete this.outboundRequests[msg.id];
      if (msg.error) {
        req.reject(new RequestError(msg.error));
      } else {
        req.resolve(msg.result);
      }
      return;
    }

    if (this.cancelled) {
      return;
    }
    this.sendResult(genericResult, msg.id, null, <RpcError>{
      code: StandardErrorCode.InvalidRequest,
      message: "has id but doesn't have method, result, or error",
    });
  }

  sendResult<Result>(
    rc: ResultCreator<Result>,
    id: number,
    result?: Result,
    error?: RpcError
  ) {
    const obj = rc(id, result, error);
    if (typeof obj.id !== "number") {
      throw new Error(`missing id in result ${JSON.stringify(obj)}`);
    }

    this.write(obj);
  }

  async call<T, U>(rc: RequestCreator<T, U>, params: T): Promise<U> {
    if (!this.firstMethod) {
      this.firstMethod = rc({} as any)(this.client).method;
    }
    return await this.internalCall(rc, params);
  }

  private async internalCall<T, U>(
    rc: RequestCreator<T, U>,
    params: T
  ): Promise<U> {
    const obj = rc(params || ({} as T))(this.client);
    if (typeof obj.id !== "number") {
      throw new Error(`missing id in request ${JSON.stringify(obj)}`);
    }

    {
      try {
        const res = await new Promise<U>((resolve, reject) => {
          this.outboundRequests[obj.id] = { resolve, reject };
          this.write(obj);
        });
        return res;
      } catch (err) {
        throw err;
      } finally {
        delete this.outboundRequests[obj.id];
      }
    }
  }

  private write(obj: any) {
    if (this.cancelled) {
      return;
    }
    let payload = JSON.stringify(obj);
    console.log("Payload = ", payload);
    this.conn.send(payload);
  }

  cancel() {
    if (this.cancelled) {
      return;
    }
    this.cancelled = true;

    for (const id of Object.keys(this.outboundRequests)) {
      let req = this.outboundRequests[parseInt(id, 10)];
      req.reject(
        RequestError.fromInternalCode(InternalCode.ConversationCancelled)
      );
    }
    this.outboundRequests = {};
    this.conn.close();
  }

  close() {
    if (this.closed) {
      return;
    }
    this.cancel();
    this.closed = true;
  }
}
