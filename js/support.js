"use strict";
// this file can be imported without pulling in node.js's "net"
// module etc., so it can be used in a browser context for example.
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
exports.__esModule = true;
exports.getRpcErrorData = exports.asRequestError = exports.getErrorStack = exports.internalCodeToString = exports.InternalCode = exports.RequestError = exports.createResult = exports.createNotification = exports.createRequest = exports.RequestType = exports.StandardErrorCode = void 0;
var StandardErrorCode;
(function (StandardErrorCode) {
    StandardErrorCode[StandardErrorCode["ParseError"] = -32700] = "ParseError";
    StandardErrorCode[StandardErrorCode["InvalidRequest"] = -32600] = "InvalidRequest";
    StandardErrorCode[StandardErrorCode["MethodNotFound"] = -32601] = "MethodNotFound";
    StandardErrorCode[StandardErrorCode["InvalidParams"] = -32602] = "InvalidParams";
    StandardErrorCode[StandardErrorCode["InternalError"] = -32603] = "InternalError";
})(StandardErrorCode = exports.StandardErrorCode || (exports.StandardErrorCode = {}));
var RequestType;
(function (RequestType) {
    RequestType[RequestType["Request"] = 0] = "Request";
    RequestType[RequestType["Notification"] = 1] = "Notification";
})(RequestType = exports.RequestType || (exports.RequestType = {}));
exports.createRequest = function (method) {
    return Object.assign(function (params) { return function (gen) { return ({
        jsonrpc: "2.0",
        method: method,
        id: gen.generateID(),
        params: params
    }); }; }, { __method: method });
};
exports.createNotification = function (method) {
    return Object.assign(function (params) { return ({
        jsonrpc: "2.0",
        method: method,
        params: params
    }); }, { __method: method });
};
exports.createResult = function () { return function (id, result, error) {
    if (error) {
        return {
            jsonrpc: "2.0",
            error: error,
            id: id
        };
    }
    else {
        return {
            jsonrpc: "2.0",
            result: result,
            id: id
        };
    }
}; };
function formatRpcError(rpcError) {
    if (rpcError.code === StandardErrorCode.InternalError) {
        // don't prefix internal errors, for readability.
        // if a `RequestError` is caught, it can still be
        // detected by checking `.rpcError`
        return rpcError.message;
    }
    return "JSON-RPC error " + rpcError.code + ": " + rpcError.message;
}
/**
 * A JavaScript Error that encapsulates a JSON-RPC 2.0 Error.
 * @see asRequestError
 * @see getErrorStack
 * @see getRpcErrorData
 */
var RequestError = /** @class */ (function (_super) {
    __extends(RequestError, _super);
    function RequestError(rpcError) {
        var _this = _super.call(this, formatRpcError(rpcError)) || this;
        _this.rpcError = rpcError;
        return _this;
    }
    RequestError.fromInternalCode = function (code) {
        return new RequestError({
            message: internalCodeToString(code),
            code: code,
            data: {
                stack: new Error().stack
            }
        });
    };
    return RequestError;
}(Error));
exports.RequestError = RequestError;
var InternalCode;
(function (InternalCode) {
    InternalCode[InternalCode["ConversationCancelled"] = -1000] = "ConversationCancelled";
    InternalCode[InternalCode["ConnectionTimedOut"] = -1100] = "ConnectionTimedOut";
    InternalCode[InternalCode["SocketClosed"] = -1200] = "SocketClosed";
})(InternalCode = exports.InternalCode || (exports.InternalCode = {}));
/**
 * Return a string representation of an internal error code.
 */
function internalCodeToString(code) {
    switch (code) {
        case InternalCode.ConversationCancelled:
            return "JSON-RPC conversation cancelled";
        case InternalCode.ConnectionTimedOut:
            return "JSON-RPC connection timed out";
        case InternalCode.SocketClosed:
            return "JSON-RPC socket closed by remote peer";
    }
}
exports.internalCodeToString = internalCodeToString;
/**
 * Get a RequestError's stack Golang trace or JavaScript
 * stack trace, if any, or message if not.
 */
function getErrorStack(e) {
    if (!e) {
        return "Unknown error";
    }
    var errorStack = e.stack || e.message;
    var re = asRequestError(e);
    if (re) {
        var ed = getRpcErrorData(e);
        if (ed && ed.stack) {
            // use golang stack if available
            errorStack = ed.stack;
        }
        else if (re.message) {
            // or just message
            errorStack = re.message;
        }
    }
    return errorStack;
}
exports.getErrorStack = getErrorStack;
/**
 * Cast an Error to `RequestError`, if it looks like one,
 * otherwise return null
 */
function asRequestError(e) {
    var re = e;
    if (re.rpcError) {
        return e;
    }
}
exports.asRequestError = asRequestError;
/**
 * If this error is a JSON-RPC 2.0 error, return its additional error data,
 * if any
 */
function getRpcErrorData(e) {
    var re = asRequestError(e);
    if (re && re.rpcError) {
        return re.rpcError.data;
    }
}
exports.getRpcErrorData = getRpcErrorData;
