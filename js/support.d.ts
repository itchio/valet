export declare enum StandardErrorCode {
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603
}
export interface IDGenerator {
    generateID(): number;
}
export interface Endpoint {
    secret: string;
    tcp: {
        address: string;
    };
}
export interface RequestCreator<Params, Result> {
    (params: Params): (gen: IDGenerator) => Request<Params, Result>;
    __method: string;
    __params?: Params;
    __result?: Params;
}
export interface NotificationCreator<Params> {
    (params: Params): Notification<Params>;
    __method: string;
    __params?: Params;
}
export declare type ResultCreator<T> = (id?: number, result?: T, error?: RpcError) => RpcResult<T>;
export declare enum RequestType {
    Request = 0,
    Notification = 1
}
export declare const createRequest: <Params, Result>(method: string) => RequestCreator<Params, Result>;
export declare const createNotification: <Params>(method: string) => NotificationCreator<Params>;
export declare const createResult: <T>() => ResultCreator<T>;
export interface Notification<T> {
    method: string;
    params: T;
}
export interface Request<T, U> extends Notification<T> {
    id: number;
}
export interface RpcResult<T> {
    jsonrpc: "2.0";
    id?: number;
    result?: T;
    error?: RpcError;
}
/**
 * A JSON-RPC 2.0 error
 */
export interface RpcError {
    code: number;
    message: string;
    data?: RpcErrorData;
}
/**
 * Additional context provided with an RPC error
 */
export interface RpcErrorData {
    stack?: string;
    butlerVersion?: string;
    apiError?: APIError;
}
/**
 * Represents an itch.io API error
 */
export interface APIError {
    messages?: string[];
    statusCode?: number;
    path?: string;
}
export interface RpcMessage {
    id: number;
    jsonrpc: string;
    method: string;
    params: any;
    error: RpcError;
    result: any;
}
/**
 * A JavaScript Error that encapsulates a JSON-RPC 2.0 Error.
 * @see asRequestError
 * @see getErrorStack
 * @see getRpcErrorData
 */
export declare class RequestError extends Error {
    rpcError: RpcError;
    constructor(rpcError: RpcError);
    static fromInternalCode(code: InternalCode): RequestError;
}
export declare enum InternalCode {
    ConversationCancelled = -1000,
    ConnectionTimedOut = -1100,
    SocketClosed = -1200
}
/**
 * Return a string representation of an internal error code.
 */
export declare function internalCodeToString(code: InternalCode): string;
/**
 * Get a RequestError's stack Golang trace or JavaScript
 * stack trace, if any, or message if not.
 */
export declare function getErrorStack(e: Error): string;
/**
 * Cast an Error to `RequestError`, if it looks like one,
 * otherwise return null
 */
export declare function asRequestError(e: Error): RequestError | undefined;
/**
 * If this error is a JSON-RPC 2.0 error, return its additional error data,
 * if any
 */
export declare function getRpcErrorData(e: Error): RpcErrorData | undefined;
export declare type RequestHandler<Params, Result> = (params: Params) => Promise<Result>;
export declare type NotificationHandler<Params> = (params: Params) => void;
export declare type ErrorHandler = (e: Error) => void;
export declare type WarningHandler = (msg: string) => void;
