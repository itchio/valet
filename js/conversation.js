"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
var support_1 = require("./support");
var _1 = require(".");
var genericResult = support_1.createResult();
var Conversation = /** @class */ (function () {
    function Conversation(client) {
        var _this = this;
        this.cancelled = false;
        this.closed = false;
        this.notificationHandlers = {};
        this.missingNotificationHandlersWarned = {};
        this.requestHandlers = {};
        this.inboundRequests = {};
        this.outboundRequests = {};
        this.client = client;
        this.conn = _1["default"].newConn();
        this.run()["catch"](function (e) {
            _this.client.warn("While processing incoming messages: " + e.stack);
        });
    }
    Conversation.prototype.run = function () {
        return __awaiter(this, void 0, void 0, function () {
            var msg, e_1;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (!true) return [3 /*break*/, 6];
                        return [4 /*yield*/, this.conn.recv()];
                    case 1:
                        msg = _a.sent();
                        _a.label = 2;
                    case 2:
                        _a.trys.push([2, 4, , 5]);
                        return [4 /*yield*/, this.handleMessage(JSON.parse(msg))];
                    case 3:
                        _a.sent();
                        return [3 /*break*/, 5];
                    case 4:
                        e_1 = _a.sent();
                        this.client.warn("While processing message: " + e_1.stack);
                        return [3 /*break*/, 5];
                    case 5: return [3 /*break*/, 0];
                    case 6: return [2 /*return*/];
                }
            });
        });
    };
    Conversation.prototype.onRequest = function (rc, handler) {
        if (this.requestHandlers[rc.__method]) {
            throw new Error("cannot register a second request handler for " + rc.__method);
        }
        this.requestHandlers[rc.__method] = handler;
    };
    Conversation.prototype.onNotification = function (nc, handler) {
        if (this.notificationHandlers[nc.__method]) {
            throw new Error("cannot register a second notification handler for " + nc.__method);
        }
        this.notificationHandlers[nc.__method] = handler;
    };
    Conversation.prototype.handleMessage = function (msg) {
        return __awaiter(this, void 0, void 0, function () {
            var handler, e_2, receivedAt, handler, result, e_3, req;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (this.cancelled) {
                            return [2 /*return*/];
                        }
                        if (typeof msg !== "object") {
                            return [2 /*return*/];
                        }
                        if (msg.jsonrpc != "2.0") {
                            return [2 /*return*/];
                        }
                        if (!(typeof msg.id === "undefined")) return [3 /*break*/, 5];
                        handler = this.notificationHandlers[msg.method];
                        if (!handler) {
                            if (!this.missingNotificationHandlersWarned[msg.method]) {
                                this.missingNotificationHandlersWarned[msg.method] = true;
                                this.client.warn("no handler for notification " + msg.method + " (in " + this.firstMethod + " convo)");
                            }
                            return [2 /*return*/];
                        }
                        _a.label = 1;
                    case 1:
                        _a.trys.push([1, 3, , 4]);
                        return [4 /*yield*/, Promise.resolve(handler(msg.params))];
                    case 2:
                        _a.sent();
                        return [3 /*break*/, 4];
                    case 3:
                        e_2 = _a.sent();
                        this.client.warn("notification handler error: " + e_2.stack);
                        if (this.client.errorHandler) {
                            this.client.errorHandler(e_2);
                        }
                        return [3 /*break*/, 4];
                    case 4: return [2 /*return*/];
                    case 5:
                        if (!msg.method) return [3 /*break*/, 13];
                        _a.label = 6;
                    case 6:
                        _a.trys.push([6, , 11, 12]);
                        this.inboundRequests[msg.id] = true;
                        receivedAt = Date.now();
                        handler = this.requestHandlers[msg.method];
                        if (!handler) {
                            if (this.cancelled) {
                                return [2 /*return*/];
                            }
                            this.sendResult(genericResult, msg.id, null, {
                                code: support_1.StandardErrorCode.MethodNotFound,
                                message: "no handler is registered for method " + msg.method
                            });
                            return [2 /*return*/];
                        }
                        _a.label = 7;
                    case 7:
                        _a.trys.push([7, 9, , 10]);
                        return [4 /*yield*/, handler(msg.params)];
                    case 8:
                        result = _a.sent();
                        if (this.cancelled) {
                            return [2 /*return*/];
                        }
                        this.sendResult(genericResult, msg.id, result, undefined);
                        return [3 /*break*/, 10];
                    case 9:
                        e_3 = _a.sent();
                        if (this.cancelled) {
                            return [2 /*return*/];
                        }
                        this.sendResult(genericResult, msg.id, null, {
                            code: support_1.StandardErrorCode.InternalError,
                            message: "async error: " + e_3.message,
                            data: {
                                stack: e_3.stack
                            }
                        });
                        return [3 /*break*/, 10];
                    case 10: return [3 /*break*/, 12];
                    case 11:
                        delete this.inboundRequests[msg.id];
                        return [7 /*endfinally*/];
                    case 12: return [2 /*return*/];
                    case 13:
                        if (msg.result || msg.error) {
                            req = this.outboundRequests[msg.id];
                            delete this.outboundRequests[msg.id];
                            if (msg.error) {
                                req.reject(new support_1.RequestError(msg.error));
                            }
                            else {
                                req.resolve(msg.result);
                            }
                            return [2 /*return*/];
                        }
                        if (this.cancelled) {
                            return [2 /*return*/];
                        }
                        this.sendResult(genericResult, msg.id, null, {
                            code: support_1.StandardErrorCode.InvalidRequest,
                            message: "has id but doesn't have method, result, or error"
                        });
                        return [2 /*return*/];
                }
            });
        });
    };
    Conversation.prototype.sendResult = function (rc, id, result, error) {
        var obj = rc(id, result, error);
        if (typeof obj.id !== "number") {
            throw new Error("missing id in result " + JSON.stringify(obj));
        }
        this.write(obj);
    };
    Conversation.prototype.call = function (rc, params) {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (!this.firstMethod) {
                            this.firstMethod = rc({})(this.client).method;
                        }
                        return [4 /*yield*/, this.internalCall(rc, params)];
                    case 1: return [2 /*return*/, _a.sent()];
                }
            });
        });
    };
    Conversation.prototype.internalCall = function (rc, params) {
        return __awaiter(this, void 0, void 0, function () {
            var obj, res, err_1;
            var _this = this;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        obj = rc(params || {})(this.client);
                        if (typeof obj.id !== "number") {
                            throw new Error("missing id in request " + JSON.stringify(obj));
                        }
                        _a.label = 1;
                    case 1:
                        _a.trys.push([1, 3, 4, 5]);
                        return [4 /*yield*/, new Promise(function (resolve, reject) {
                                _this.outboundRequests[obj.id] = { resolve: resolve, reject: reject };
                                _this.write(obj);
                            })];
                    case 2:
                        res = _a.sent();
                        return [2 /*return*/, res];
                    case 3:
                        err_1 = _a.sent();
                        throw err_1;
                    case 4:
                        delete this.outboundRequests[obj.id];
                        return [7 /*endfinally*/];
                    case 5: return [2 /*return*/];
                }
            });
        });
    };
    Conversation.prototype.write = function (obj) {
        if (this.cancelled) {
            return;
        }
        var payload = JSON.stringify(obj);
        console.log("Payload = ", payload);
        this.conn.send(payload);
    };
    Conversation.prototype.cancel = function () {
        if (this.cancelled) {
            return;
        }
        this.cancelled = true;
        for (var _i = 0, _a = Object.keys(this.outboundRequests); _i < _a.length; _i++) {
            var id = _a[_i];
            var req = this.outboundRequests[parseInt(id, 10)];
            req.reject(support_1.RequestError.fromInternalCode(support_1.InternalCode.ConversationCancelled));
        }
        this.outboundRequests = {};
        this.conn.close();
    };
    Conversation.prototype.close = function () {
        if (this.closed) {
            return;
        }
        this.cancel();
        this.closed = true;
    };
    return Conversation;
}());
exports.Conversation = Conversation;
