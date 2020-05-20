import { NotificationHandler, RequestHandler, RequestCreator, NotificationCreator, RpcError, ResultCreator } from "./support";
import { Client } from "./client";
export declare class Conversation {
    private cancelled;
    private conn;
    private closed;
    private notificationHandlers;
    private missingNotificationHandlersWarned;
    private requestHandlers;
    private client;
    private inboundRequests;
    private outboundRequests;
    private firstMethod?;
    constructor(client: Client);
    private run;
    onRequest<Params, Result>(rc: RequestCreator<Params, Result>, handler: RequestHandler<Params, Result>): void;
    onNotification<T>(nc: NotificationCreator<T>, handler: NotificationHandler<T>): void;
    private handleMessage;
    sendResult<Result>(rc: ResultCreator<Result>, id: number, result?: Result, error?: RpcError): void;
    call<T, U>(rc: RequestCreator<T, U>, params: T): Promise<U>;
    private internalCall;
    private write;
    cancel(): void;
    close(): void;
}
