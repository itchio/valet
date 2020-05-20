import { ErrorHandler, WarningHandler, RequestCreator } from "./support";
import { Conversation } from "./conversation";
export declare type SetupFunc = (c: Conversation) => void;
export declare class Client {
    errorHandler?: ErrorHandler;
    warningHandler?: WarningHandler;
    idSeed: number;
    constructor();
    generateID(): number;
    warn(msg: string): void;
    onError(handler: ErrorHandler): void;
    onWarning(handler: WarningHandler): void;
    call<Params, Result>(rc: RequestCreator<Params, Result>, params: Params, setup?: SetupFunc): Promise<Result>;
}
