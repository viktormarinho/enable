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
Object.defineProperty(exports, "__esModule", { value: true });
exports.EnableClient = void 0;
class EnableClient {
    constructor(config) {
        this.baseUrl = config.url;
        this.token = config.token;
    }
    getStatus(featureId) {
        return __awaiter(this, void 0, void 0, function* () {
            const res = yield fetch(`${this.baseUrl}/api/feature/${featureId}`, {
                headers: {
                    Authorization: `Bearer ${this.token}`,
                },
            });
            return yield res.json();
        });
    }
    isActive(featureId) {
        return __awaiter(this, void 0, void 0, function* () {
            const { active } = yield this.getStatus(featureId);
            return active;
        });
    }
    isDisabled(featureId) {
        return __awaiter(this, void 0, void 0, function* () {
            const { active } = yield this.getStatus(featureId);
            return !active;
        });
    }
}
exports.EnableClient = EnableClient;
