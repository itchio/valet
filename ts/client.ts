import { ErrorHandler, WarningHandler, RequestCreator } from "./support";
import { Conversation } from "./conversation";

export type SetupFunc = (c: Conversation) => void;

export class Client {
  errorHandler?: ErrorHandler;
  warningHandler?: WarningHandler;

  idSeed = 1;

  constructor() {}

  generateID(): number {
    return this.idSeed++;
  }

  warn(msg: string) {
    if (this.warningHandler) {
      try {
        this.warningHandler(msg);
        return;
      } catch (e) {}
    }
    console.warn(msg);
  }

  onError(handler: ErrorHandler) {
    this.errorHandler = handler;
  }

  onWarning(handler: WarningHandler) {
    this.warningHandler = handler;
  }

  async call<Params, Result>(
    rc: RequestCreator<Params, Result>,
    params: Params,
    setup?: SetupFunc
  ): Promise<Result> {
    let conversation = new Conversation(this);
    try {
      if (setup) {
        setup(conversation);
      }
      let res = await conversation.call(rc, params);
      return res;
    } finally {
      conversation.close();
    }
  }
}
