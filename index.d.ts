
interface NewServerOpts {
    /** path to DB file */
    dbPath: string;

    /** user agent */
    userAgent?: string;

    /** itch.io API address, defaults to "https://itch.io" */
    address?: string;
}

interface Valet {
    newServer(opts: NewServerOpts): Server;
};

interface Server {
    send(payload: string);
    // TODO: promisify
    recv(): string;
};

declare let valet: Valet;
export = valet;
