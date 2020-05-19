export interface InitOpts {
    /** path to DB file */
    dbPath: string;
    /** user agent */
    userAgent?: string;
    /** itch.io API address, defaults to "https://itch.io" */
    address?: string;
}
export interface Conn {
    send(payload: string): void;
    recv(): Promise<string>;
    close(): void;
}
export interface VersionObject {
    major: number;
    minor: number;
    patch: number;
}
export interface ValetStatic {
    version: VersionObject;
    initialize(opts: InitOpts): void;
    newConn(): Conn;
    goPanic(): void;
    rustPanic(): void;
}
declare let valet: ValetStatic;
export default valet;
