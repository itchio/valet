export declare type RFCDate = string;
/**
 * undocumented
 */
export interface LaunchTarget {
    /**
     * The manifest action corresponding to this launch target.
     * For implicit launch targets, a minimal one will be generated.
     */
    action: Action;
    /** Host this launch target was found for */
    host: Host;
    /** Detailed launch strategy */
    strategy: StrategyResult;
}
/**
 */
export interface StrategyResult {
    /** Name of launch strategy used for launch target */
    strategy: LaunchStrategy;
    /** Absolute filesystem path of the target. */
    fullTargetPath: string;
    /** If a local file, result of dash configure */
    candidate: Candidate;
}
/**
 * undocumented
 */
export declare enum LaunchStrategy {
    Unknown = "",
    Native = "native",
    HTML = "html",
    URL = "url",
    Shell = "shell"
}
/**
 * Result for Meta.Authenticate
 */
export interface MetaAuthenticateResult {
    /** undocumented */
    ok: boolean;
}
/**
 * When using TCP transport, must be the first message sent
 */
export declare const MetaAuthenticate: import("./support").RequestCreator<MetaAuthenticateParams, MetaAuthenticateResult>;
/**
 * Result for Meta.Flow
 */
export interface MetaFlowResult {
}
/**
 * When called, defines the entire duration of the daemon's life.
 *
 * Cancelling that conversation (or closing the TCP connection) will
 * shut down the daemon after all other requests have finished. This
 * allows gracefully switching to another daemon.
 *
 * This conversation is also used to send all global notifications,
 * regarding data that's fetched, network state, etc.
 *
 * Note that this call never returns - you have to cancel it when you're
 * done with the daemon.
 */
export declare const MetaFlow: import("./support").RequestCreator<MetaFlowParams, MetaFlowResult>;
/**
 * Result for Meta.Shutdown
 */
export interface MetaShutdownResult {
}
/**
 * When called, gracefully shutdown the butler daemon.
 */
export declare const MetaShutdown: import("./support").RequestCreator<MetaShutdownParams, MetaShutdownResult>;
/**
 * Result for Version.Get
 */
export interface VersionGetResult {
    /** Something short, like `v8.0.0` */
    version: string;
    /** Something long, like `v8.0.0, built on Aug 27 2017 @ 01:13:55, ref d833cc0aeea81c236c81dffb27bc18b2b8d8b290` */
    versionString: string;
}
/**
 * Retrieves the version of the butler instance the client
 * is connected to.
 *
 * This endpoint is meant to gather information when reporting
 * issues, rather than feature sniffing. Conforming clients should
 * automatically download new versions of butler, see the **Updating** section.
 */
export declare const VersionGet: import("./support").RequestCreator<VersionGetParams, VersionGetResult>;
/**
 * Result for Network.SetSimulateOffline
 */
export interface NetworkSetSimulateOfflineResult {
}
/**
 * undocumented
 */
export declare const NetworkSetSimulateOffline: import("./support").RequestCreator<NetworkSetSimulateOfflineParams, NetworkSetSimulateOfflineResult>;
/**
 * Result for Network.SetBandwidthThrottle
 */
export interface NetworkSetBandwidthThrottleResult {
}
/**
 * undocumented
 */
export declare const NetworkSetBandwidthThrottle: import("./support").RequestCreator<NetworkSetBandwidthThrottleParams, NetworkSetBandwidthThrottleResult>;
/**
 * Result for Profile.List
 */
export interface ProfileListResult {
    /** A list of remembered profiles */
    profiles: Profile[];
}
/**
 * Lists remembered profiles
 */
export declare const ProfileList: import("./support").RequestCreator<ProfileListParams, ProfileListResult>;
/**
 * Represents a user for which we have profile information,
 * ie. that we can connect as, etc.
 */
export interface Profile {
    /** itch.io user ID, doubling as profile ID */
    id: number;
    /** Timestamp the user last connected at (to the client) */
    lastConnected: RFCDate;
    /** User information */
    user: User;
}
/**
 * Result for Profile.LoginWithPassword
 */
export interface ProfileLoginWithPasswordResult {
    /** Information for the new profile, now remembered */
    profile: Profile;
    /** Profile cookie for website */
    cookie: {
        [key: string]: string;
    };
}
/**
 * Add a new profile by password login
 */
export declare const ProfileLoginWithPassword: import("./support").RequestCreator<ProfileLoginWithPasswordParams, ProfileLoginWithPasswordResult>;
/**
 * Result for Profile.LoginWithAPIKey
 */
export interface ProfileLoginWithAPIKeyResult {
    /** Information for the new profile, now remembered */
    profile: Profile;
}
/**
 * Add a new profile by API key login. This can be used
 * for integration tests, for example. Note that no cookies
 * are returned for this kind of login.
 */
export declare const ProfileLoginWithAPIKey: import("./support").RequestCreator<ProfileLoginWithAPIKeyParams, ProfileLoginWithAPIKeyResult>;
/**
 * Result for Profile.RequestCaptcha
 */
export interface ProfileRequestCaptchaResult {
    /** The response given by recaptcha after it's been filled */
    recaptchaResponse: string;
}
/**
 * Ask the user to solve a captcha challenge
 * Sent during @@ProfileLoginWithPasswordParams if certain
 * conditions are met.
 */
export declare const ProfileRequestCaptcha: import("./support").RequestCreator<ProfileRequestCaptchaParams, ProfileRequestCaptchaResult>;
/**
 * Result for Profile.RequestTOTP
 */
export interface ProfileRequestTOTPResult {
    /** The TOTP code entered by the user */
    code: string;
}
/**
 * Ask the user to provide a TOTP token.
 * Sent during @@ProfileLoginWithPasswordParams if the user has
 * two-factor authentication enabled.
 */
export declare const ProfileRequestTOTP: import("./support").RequestCreator<ProfileRequestTOTPParams, ProfileRequestTOTPResult>;
/**
 * Result for Profile.UseSavedLogin
 */
export interface ProfileUseSavedLoginResult {
    /** Information for the now validated profile */
    profile: Profile;
}
/**
 * Use saved login credentials to validate a profile.
 */
export declare const ProfileUseSavedLogin: import("./support").RequestCreator<ProfileUseSavedLoginParams, ProfileUseSavedLoginResult>;
/**
 * Result for Profile.Forget
 */
export interface ProfileForgetResult {
    /** True if the profile did exist (and was successfully forgotten) */
    success: boolean;
}
/**
 * Forgets a remembered profile - it won't appear in the
 * @@ProfileListParams results anymore.
 */
export declare const ProfileForget: import("./support").RequestCreator<ProfileForgetParams, ProfileForgetResult>;
/**
 * Result for Profile.Data.Put
 */
export interface ProfileDataPutResult {
}
/**
 * Stores some data associated to a profile, by key.
 */
export declare const ProfileDataPut: import("./support").RequestCreator<ProfileDataPutParams, ProfileDataPutResult>;
/**
 * Result for Profile.Data.Get
 */
export interface ProfileDataGetResult {
    /** True if the value existed */
    ok: boolean;
    /** undocumented */
    value: string;
}
/**
 * Retrieves some data associated to a profile, by key.
 */
export declare const ProfileDataGet: import("./support").RequestCreator<ProfileDataGetParams, ProfileDataGetResult>;
/**
 * Result for Search.Games
 */
export interface SearchGamesResult {
    /** undocumented */
    games: Game[];
}
/**
 * Searches for games.
 */
export declare const SearchGames: import("./support").RequestCreator<SearchGamesParams, SearchGamesResult>;
/**
 * Result for Search.Users
 */
export interface SearchUsersResult {
    /** undocumented */
    users: User[];
}
/**
 * Searches for users.
 */
export declare const SearchUsers: import("./support").RequestCreator<SearchUsersParams, SearchUsersResult>;
/**
 * Result for Fetch.Game
 */
export interface FetchGameResult {
    /** Game info */
    game: Game;
    /** Marks that a request should be issued afterwards with 'Fresh' set */
    stale?: boolean;
}
/**
 * Fetches information for an itch.io game.
 */
export declare const FetchGame: import("./support").RequestCreator<FetchGameParams, FetchGameResult>;
/**
 * undocumented
 */
export interface GameRecord {
    /** Game ID */
    id: number;
    /** Game title */
    title: string;
    /** Game cover */
    cover: string;
    /** True if owned */
    owned: boolean;
    /** Non-nil if installed (has caves) */
    installedAt: RFCDate;
}
/**
 * undocumented
 */
export declare enum GameRecordsSource {
    Owned = "owned",
    Installed = "installed",
    Profile = "profile",
    Collection = "collection"
}
/**
 * undocumented
 */
export interface GameRecordsFilters {
    /** undocumented */
    classification?: GameClassification;
    /** undocumented */
    installed?: boolean;
    /** undocumented */
    owned?: boolean;
}
/**
 * Result for Fetch.GameRecords
 */
export interface FetchGameRecordsResult {
    /** All the records that were fetched */
    records: GameRecord[];
    /** Marks that a request should be issued afterwards with 'Fresh' set */
    stale?: boolean;
}
/**
 * Fetches game records - owned, installed, in collection,
 * with search, etc. Includes download key info, cave info, etc.
 */
export declare const FetchGameRecords: import("./support").RequestCreator<FetchGameRecordsParams, FetchGameRecordsResult>;
/**
 * Result for Fetch.DownloadKey
 */
export interface FetchDownloadKeyResult {
    /** undocumented */
    downloadKey: DownloadKey;
    /** Marks that a request should be issued afterwards with 'Fresh' set */
    stale?: boolean;
}
/**
 * Fetches a download key
 */
export declare const FetchDownloadKey: import("./support").RequestCreator<FetchDownloadKeyParams, FetchDownloadKeyResult>;
/**
 * undocumented
 */
export interface FetchDownloadKeysFilter {
    /** Return only download keys for given game */
    gameId?: number;
}
/**
 * Result for Fetch.DownloadKeys
 */
export interface FetchDownloadKeysResult {
    /** All the download keys found in the local DB. */
    items: DownloadKey[];
    /**
     * Whether the information was fetched from a stale cache,
     * and could warrant a refresh if online.
     */
    stale: boolean;
}
/**
 * Fetches multiple download keys
 */
export declare const FetchDownloadKeys: import("./support").RequestCreator<FetchDownloadKeysParams, FetchDownloadKeysResult>;
/**
 * Result for Fetch.GameUploads
 */
export interface FetchGameUploadsResult {
    /** List of uploads */
    uploads: Upload[];
    /**
     * Marks that a request should be issued
     * afterwards with 'Fresh' set
     */
    stale?: boolean;
}
/**
 * Fetches uploads for an itch.io game
 */
export declare const FetchGameUploads: import("./support").RequestCreator<FetchGameUploadsParams, FetchGameUploadsResult>;
/**
 * Result for Fetch.User
 */
export interface FetchUserResult {
    /** User info */
    user: User;
    /**
     * Marks that a request should be issued
     * afterwards with 'Fresh' set
     */
    stale?: boolean;
}
/**
 * Fetches information for an itch.io user.
 */
export declare const FetchUser: import("./support").RequestCreator<FetchUserParams, FetchUserResult>;
/**
 * Result for Fetch.Sale
 */
export interface FetchSaleResult {
    /** undocumented */
    sale?: Sale;
}
/**
 * Fetches the best current *locally cached* sale for a given
 * game.
 */
export declare const FetchSale: import("./support").RequestCreator<FetchSaleParams, FetchSaleResult>;
/**
 * Result for Fetch.Collection
 */
export interface FetchCollectionResult {
    /** Collection info */
    collection: Collection;
    /**
     * True if the info was from local DB and
     * it should be re-queried using "Fresh"
     */
    stale?: boolean;
}
/**
 * Fetch a collection's title, gamesCount, etc.
 * but not its games.
 */
export declare const FetchCollection: import("./support").RequestCreator<FetchCollectionParams, FetchCollectionResult>;
/**
 * undocumented
 */
export interface CollectionGamesFilters {
    /** undocumented */
    installed: boolean;
    /** undocumented */
    classification: GameClassification;
}
/**
 * Result for Fetch.Collection.Games
 */
export interface FetchCollectionGamesResult {
    /** Requested games for this collection */
    items: CollectionGame[];
    /** Use to fetch the next 'page' of results */
    nextCursor?: Cursor;
    /** If true, re-issue request with 'Fresh' */
    stale?: boolean;
}
/**
 * Fetches information about a collection and the games it
 * contains.
 */
export declare const FetchCollectionGames: import("./support").RequestCreator<FetchCollectionGamesParams, FetchCollectionGamesResult>;
/**
 * Result for Fetch.ProfileCollections
 */
export interface FetchProfileCollectionsResult {
    /** Collections belonging to the profile */
    items: Collection[];
    /** Used to fetch the next page */
    nextCursor?: Cursor;
    /** If true, re-issue request with "Fresh" */
    stale?: boolean;
}
/**
 * Lists collections for a profile. Does not contain
 * games.
 */
export declare const FetchProfileCollections: import("./support").RequestCreator<FetchProfileCollectionsParams, FetchProfileCollectionsResult>;
/**
 * undocumented
 */
export interface ProfileGameFilters {
    /** undocumented */
    visibility: string;
    /** undocumented */
    paidStatus: string;
}
/**
 * undocumented
 */
export interface ProfileGame {
    /** undocumented */
    game: Game;
    /** undocumented */
    viewsCount: number;
    /** undocumented */
    downloadsCount: number;
    /** undocumented */
    purchasesCount: number;
    /** undocumented */
    published: boolean;
}
/**
 * Result for Fetch.ProfileGames
 */
export interface FetchProfileGamesResult {
    /** Profile games */
    items: ProfileGame[];
    /** Used to fetch the next page */
    nextCursor?: Cursor;
    /** If true, re-issue request with "Fresh" */
    stale?: boolean;
}
/**
 * undocumented
 */
export declare const FetchProfileGames: import("./support").RequestCreator<FetchProfileGamesParams, FetchProfileGamesResult>;
/**
 * undocumented
 */
export interface ProfileOwnedKeysFilters {
    /** undocumented */
    installed: boolean;
    /** undocumented */
    classification: GameClassification;
}
/**
 * Result for Fetch.ProfileOwnedKeys
 */
export interface FetchProfileOwnedKeysResult {
    /** Download keys fetched for profile */
    items: DownloadKey[];
    /** Used to fetch the next page */
    nextCursor?: Cursor;
    /** If true, re-issue request with "Fresh" */
    stale?: boolean;
}
/**
 * undocumented
 */
export declare const FetchProfileOwnedKeys: import("./support").RequestCreator<FetchProfileOwnedKeysParams, FetchProfileOwnedKeysResult>;
/**
 * Result for Fetch.Commons
 */
export interface FetchCommonsResult {
    /** undocumented */
    downloadKeys: DownloadKeySummary[];
    /** undocumented */
    caves: CaveSummary[];
    /** undocumented */
    installLocations: InstallLocationSummary[];
}
/**
 * undocumented
 */
export declare const FetchCommons: import("./support").RequestCreator<FetchCommonsParams, FetchCommonsResult>;
/**
 * undocumented
 */
export interface DownloadKeySummary {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Identifier of the game to which this download key grants access */
    gameId: number;
    /** Date this key was created at (often coincides with purchase time) */
    createdAt: RFCDate;
}
/**
 * undocumented
 */
export interface CaveSummary {
    /** undocumented */
    id: string;
    /** undocumented */
    gameId: number;
    /** undocumented */
    lastTouchedAt: RFCDate;
    /** undocumented */
    secondsRun: number;
    /** undocumented */
    installedSize: number;
}
/**
 * A Cave corresponds to an "installed item" for a game.
 *
 * It maps one-to-one with an upload. There might be 0, 1, or several
 * caves for a given game. Multiple caves for a single game is a rare-ish
 * case (single-page bundles, bonus content) but one that should be handled.
 */
export interface Cave {
    /** Unique identifier of this cave (UUID) */
    id: string;
    /** Game that's installed in this cave */
    game: Game;
    /** Upload that's installed in this cave */
    upload: Upload;
    /** Build that's installed in this cave, if the upload is wharf-powered */
    build?: Build;
    /** Stats about cave usage and first install */
    stats: CaveStats;
    /** Information about where the cave is installed, how much space it takes up etc. */
    installInfo: CaveInstallInfo;
}
/**
 * CaveStats contains stats about cave usage and first install
 */
export interface CaveStats {
    /** Time the cave was first installed */
    installedAt: RFCDate;
    /** undocumented */
    lastTouchedAt: RFCDate;
    /** undocumented */
    secondsRun: number;
}
/**
 * CaveInstallInfo contains information about where the cave is installed, how
 * much space it takes up, etc.
 */
export interface CaveInstallInfo {
    /**
     * Size the cave takes up - or at least, size it took up when we finished
     * installing it. Does not include files generated by the game in the install folder.
     */
    installedSize: number;
    /**
     * Name of the install location for this cave. This may change if the cave
     * is moved.
     */
    installLocation: string;
    /** Absolute path to the install folder */
    installFolder: string;
    /** If true, this cave is ignored while checking for updates */
    pinned: boolean;
}
/**
 * undocumented
 */
export interface InstallLocationSummary {
    /** Unique identifier for this install location */
    id: string;
    /** Absolute path on disk for this install location */
    path: string;
    /** Information about the size used and available at this install location */
    sizeInfo: InstallLocationSizeInfo;
}
/**
 * undocumented
 */
export interface InstallLocationSizeInfo {
    /** Number of bytes used by caves installed in this location */
    installedSize: number;
    /**
     * Free space at this location (depends on the partition/disk on which
     * it is), or a negative value if we can't find it
     */
    freeSize: number;
    /**
     * Total space of this location (depends on the partition/disk on which
     * it is), or a negative value if we can't find it
     */
    totalSize: number;
}
/**
 * undocumented
 */
export interface CavesFilters {
    /** undocumented */
    classification?: GameClassification;
    /** undocumented */
    gameId?: number;
    /** undocumented */
    installLocationId?: string;
}
/**
 * Result for Fetch.Caves
 */
export interface FetchCavesResult {
    /** undocumented */
    items: Cave[];
    /** Use to fetch the next 'page' of results */
    nextCursor?: Cursor;
}
/**
 * Retrieve info for all caves.
 */
export declare const FetchCaves: import("./support").RequestCreator<FetchCavesParams, FetchCavesResult>;
/**
 * Result for Fetch.Cave
 */
export interface FetchCaveResult {
    /** undocumented */
    cave: Cave;
}
/**
 * Retrieve info on a cave by ID.
 */
export declare const FetchCave: import("./support").RequestCreator<FetchCaveParams, FetchCaveResult>;
/**
 * Result for Fetch.ExpireAll
 */
export interface FetchExpireAllResult {
}
/**
 * Mark all local data as stale.
 */
export declare const FetchExpireAll: import("./support").RequestCreator<FetchExpireAllParams, FetchExpireAllResult>;
/**
 * Result for Game.FindUploads
 */
export interface GameFindUploadsResult {
    /** A list of uploads that were found to be compatible. */
    uploads: Upload[];
}
/**
 * Finds uploads compatible with the current runtime, for a given game.
 */
export declare const GameFindUploads: import("./support").RequestCreator<GameFindUploadsParams, GameFindUploadsResult>;
/**
 * Result for Install.Queue
 */
export interface InstallQueueResult {
    /** undocumented */
    id: string;
    /** undocumented */
    reason: DownloadReason;
    /** undocumented */
    caveId: string;
    /** undocumented */
    game: Game;
    /** undocumented */
    upload: Upload;
    /** undocumented */
    build: Build;
    /** undocumented */
    installFolder: string;
    /** undocumented */
    stagingFolder: string;
    /** undocumented */
    installLocationId: string;
}
/**
 * Queues an install operation to be later performed
 * via @@InstallPerformParams.
 */
export declare const InstallQueue: import("./support").RequestCreator<InstallQueueParams, InstallQueueResult>;
/**
 * Result for Install.Plan
 */
export interface InstallPlanResult {
    /** undocumented */
    game: Game;
    /** undocumented */
    uploads: Upload[];
    /** undocumented */
    info: InstallPlanInfo;
}
/**
 * For modal-first install
 */
export declare const InstallPlan: import("./support").RequestCreator<InstallPlanParams, InstallPlanResult>;
/**
 * undocumented
 */
export interface InstallPlanInfo {
    /** undocumented */
    upload: Upload;
    /** undocumented */
    build: Build;
    /** undocumented */
    type: string;
    /** undocumented */
    diskUsage: DiskUsageInfo;
    /** undocumented */
    error: string;
    /** undocumented */
    errorMessage: string;
    /** undocumented */
    errorCode: number;
}
/**
 * undocumented
 */
export interface DiskUsageInfo {
    /** undocumented */
    finalDiskUsage: number;
    /** undocumented */
    neededFreeSpace: number;
    /** undocumented */
    accuracy: string;
}
/**
 * Result for Caves.SetPinned
 */
export interface CavesSetPinnedResult {
}
/**
 * undocumented
 */
export declare const CavesSetPinned: import("./support").RequestCreator<CavesSetPinnedParams, CavesSetPinnedResult>;
/**
 * Result for Install.CreateShortcut
 */
export interface InstallCreateShortcutResult {
}
/**
 * Create a shortcut for an existing cave .
 */
export declare const InstallCreateShortcut: import("./support").RequestCreator<InstallCreateShortcutParams, InstallCreateShortcutResult>;
/**
 * Result for Install.Perform
 */
export interface InstallPerformResult {
    /** undocumented */
    caveId: string;
    /** undocumented */
    events: InstallEvent[];
}
/**
 * Perform an install that was previously queued via
 * @@InstallQueueParams.
 *
 * Can be cancelled by passing the same `ID` to @@InstallCancelParams.
 */
export declare const InstallPerform: import("./support").RequestCreator<InstallPerformParams, InstallPerformResult>;
/**
 * Result for Install.Cancel
 */
export interface InstallCancelResult {
    /** undocumented */
    didCancel: boolean;
}
/**
 * Attempt to gracefully cancel an ongoing operation.
 */
export declare const InstallCancel: import("./support").RequestCreator<InstallCancelParams, InstallCancelResult>;
/**
 * Result for Uninstall.Perform
 */
export interface UninstallPerformResult {
}
/**
 * UninstallParams contains all the parameters needed to perform
 * an uninstallation for a game via @@OperationStartParams.
 */
export declare const UninstallPerform: import("./support").RequestCreator<UninstallPerformParams, UninstallPerformResult>;
/**
 * Result for Install.VersionSwitch.Queue
 */
export interface InstallVersionSwitchQueueResult {
}
/**
 * Prepare to queue a version switch. The client will
 * receive an @@InstallVersionSwitchPickParams.
 */
export declare const InstallVersionSwitchQueue: import("./support").RequestCreator<InstallVersionSwitchQueueParams, InstallVersionSwitchQueueResult>;
/**
 * Result for InstallVersionSwitchPick
 */
export interface InstallVersionSwitchPickResult {
    /** A negative index aborts the version switch */
    index: number;
}
/**
 * Let the user pick which version to switch to.
 */
export declare const InstallVersionSwitchPick: import("./support").RequestCreator<InstallVersionSwitchPickParams, InstallVersionSwitchPickResult>;
/**
 * GameCredentials contains all the credentials required to make API requests
 * including the download key if any.
 */
export interface GameCredentials {
    /** A valid itch.io API key */
    apiKey: string;
    /** A download key identifier, or 0 if no download key is available */
    downloadKey?: number;
}
/**
 * Result for PickUpload
 */
export interface PickUploadResult {
    /**
     * The index (in the original array) of the upload that was picked,
     * or a negative value to cancel.
     */
    index: number;
}
/**
 * Asks the user to pick between multiple available uploads
 */
export declare const PickUpload: import("./support").RequestCreator<PickUploadParams, PickUploadResult>;
/**
 * Result for Install.Locations.List
 */
export interface InstallLocationsListResult {
    /** undocumented */
    installLocations: InstallLocationSummary[];
}
/**
 * undocumented
 */
export declare const InstallLocationsList: import("./support").RequestCreator<InstallLocationsListParams, InstallLocationsListResult>;
/**
 * Result for Install.Locations.Add
 */
export interface InstallLocationsAddResult {
    /** undocumented */
    installLocation: InstallLocationSummary;
}
/**
 * undocumented
 */
export declare const InstallLocationsAdd: import("./support").RequestCreator<InstallLocationsAddParams, InstallLocationsAddResult>;
/**
 * Result for Install.Locations.Remove
 */
export interface InstallLocationsRemoveResult {
}
/**
 * undocumented
 */
export declare const InstallLocationsRemove: import("./support").RequestCreator<InstallLocationsRemoveParams, InstallLocationsRemoveResult>;
/**
 * Result for Install.Locations.GetByID
 */
export interface InstallLocationsGetByIDResult {
    /** undocumented */
    installLocation: InstallLocationSummary;
}
/**
 * undocumented
 */
export declare const InstallLocationsGetByID: import("./support").RequestCreator<InstallLocationsGetByIDParams, InstallLocationsGetByIDResult>;
/**
 * Result for Install.Locations.Scan.ConfirmImport
 */
export interface InstallLocationsScanConfirmImportResult {
    /** undocumented */
    confirm: boolean;
}
/**
 * Sent at the end of @@InstallLocationsScanParams
 */
export declare const InstallLocationsScanConfirmImport: import("./support").RequestCreator<InstallLocationsScanConfirmImportParams, InstallLocationsScanConfirmImportResult>;
/**
 * Result for Install.Locations.Scan
 */
export interface InstallLocationsScanResult {
    /** undocumented */
    numFoundItems: number;
    /** undocumented */
    numImportedItems: number;
}
/**
 * undocumented
 */
export declare const InstallLocationsScan: import("./support").RequestCreator<InstallLocationsScanParams, InstallLocationsScanResult>;
/**
 * Result for Downloads.Queue
 */
export interface DownloadsQueueResult {
}
/**
 * Queue a download that will be performed later by
 * @@DownloadsDriveParams.
 */
export declare const DownloadsQueue: import("./support").RequestCreator<DownloadsQueueParams, DownloadsQueueResult>;
/**
 * Result for Downloads.Prioritize
 */
export interface DownloadsPrioritizeResult {
}
/**
 * Put a download on top of the queue.
 */
export declare const DownloadsPrioritize: import("./support").RequestCreator<DownloadsPrioritizeParams, DownloadsPrioritizeResult>;
/**
 * Result for Downloads.List
 */
export interface DownloadsListResult {
    /** undocumented */
    downloads: Download[];
}
/**
 * List all known downloads.
 */
export declare const DownloadsList: import("./support").RequestCreator<DownloadsListParams, DownloadsListResult>;
/**
 * Result for Downloads.ClearFinished
 */
export interface DownloadsClearFinishedResult {
}
/**
 * Removes all finished downloads from the queue.
 */
export declare const DownloadsClearFinished: import("./support").RequestCreator<DownloadsClearFinishedParams, DownloadsClearFinishedResult>;
/**
 * Result for Downloads.Drive
 */
export interface DownloadsDriveResult {
}
/**
 * Drive downloads, which is: perform them one at a time,
 * until they're all finished.
 */
export declare const DownloadsDrive: import("./support").RequestCreator<DownloadsDriveParams, DownloadsDriveResult>;
/**
 * Result for Downloads.Drive.Cancel
 */
export interface DownloadsDriveCancelResult {
    /** undocumented */
    didCancel: boolean;
}
/**
 * Stop driving downloads gracefully.
 */
export declare const DownloadsDriveCancel: import("./support").RequestCreator<DownloadsDriveCancelParams, DownloadsDriveCancelResult>;
/**
 * Payload for Downloads.Drive.Progress
 */
export interface DownloadsDriveProgressNotification {
    /** undocumented */
    download: Download;
    /** undocumented */
    progress: DownloadProgress;
    /** BPS values for the last minute */
    speedHistory: number[];
}
/**
 * undocumented
 */
export declare const DownloadsDriveProgress: import("./support").NotificationCreator<DownloadsDriveProgressNotification>;
/**
 * Payload for Downloads.Drive.Started
 */
export interface DownloadsDriveStartedNotification {
    /** undocumented */
    download: Download;
}
/**
 * undocumented
 */
export declare const DownloadsDriveStarted: import("./support").NotificationCreator<DownloadsDriveStartedNotification>;
/**
 * Payload for Downloads.Drive.Errored
 */
export interface DownloadsDriveErroredNotification {
    /**
     * The download that errored. It contains all the error
     * information: a short message, a full stack trace,
     * and a butlerd error code.
     */
    download: Download;
}
/**
 * undocumented
 */
export declare const DownloadsDriveErrored: import("./support").NotificationCreator<DownloadsDriveErroredNotification>;
/**
 * Payload for Downloads.Drive.Finished
 */
export interface DownloadsDriveFinishedNotification {
    /** undocumented */
    download: Download;
}
/**
 * undocumented
 */
export declare const DownloadsDriveFinished: import("./support").NotificationCreator<DownloadsDriveFinishedNotification>;
/**
 * Payload for Downloads.Drive.Discarded
 */
export interface DownloadsDriveDiscardedNotification {
    /** undocumented */
    download: Download;
}
/**
 * undocumented
 */
export declare const DownloadsDriveDiscarded: import("./support").NotificationCreator<DownloadsDriveDiscardedNotification>;
/**
 * Payload for Downloads.Drive.NetworkStatus
 */
export interface DownloadsDriveNetworkStatusNotification {
    /** The current network status */
    status: NetworkStatus;
}
/**
 * Sent during @@DownloadsDriveParams to inform on network
 * status changes.
 */
export declare const DownloadsDriveNetworkStatus: import("./support").NotificationCreator<DownloadsDriveNetworkStatusNotification>;
/**
 * undocumented
 */
export declare enum NetworkStatus {
    Online = "online",
    Offline = "offline"
}
/**
 * undocumented
 */
export declare enum DownloadReason {
    Install = "install",
    Reinstall = "reinstall",
    Update = "update",
    VersionSwitch = "version-switch"
}
/**
 * Represents a download queued, which will be
 * performed whenever @@DownloadsDriveParams is called.
 */
export interface Download {
    /** undocumented */
    id: string;
    /** undocumented */
    error: string;
    /** undocumented */
    errorMessage: string;
    /** undocumented */
    errorCode: number;
    /** undocumented */
    reason: DownloadReason;
    /** undocumented */
    position: number;
    /** undocumented */
    caveId: string;
    /** undocumented */
    game: Game;
    /** undocumented */
    upload: Upload;
    /** undocumented */
    build: Build;
    /** undocumented */
    startedAt: RFCDate;
    /** undocumented */
    finishedAt: RFCDate;
    /** undocumented */
    stagingFolder: string;
}
/**
 * undocumented
 */
export interface DownloadProgress {
    /** undocumented */
    stage: string;
    /** undocumented */
    progress: number;
    /** undocumented */
    eta: number;
    /** undocumented */
    bps: number;
}
/**
 * Result for Downloads.Retry
 */
export interface DownloadsRetryResult {
}
/**
 * Retries a download that has errored
 */
export declare const DownloadsRetry: import("./support").RequestCreator<DownloadsRetryParams, DownloadsRetryResult>;
/**
 * Result for Downloads.Discard
 */
export interface DownloadsDiscardResult {
}
/**
 * Attempts to discard a download
 */
export declare const DownloadsDiscard: import("./support").RequestCreator<DownloadsDiscardParams, DownloadsDiscardResult>;
/**
 * Result for CheckUpdate
 */
export interface CheckUpdateResult {
    /** Any updates found (might be empty) */
    updates: GameUpdate[];
    /** Warnings messages logged while looking for updates */
    warnings: string[];
}
/**
 * Looks for game updates.
 *
 * If a list of cave identifiers is passed, will only look for
 * updates for these caves *and will ignore snooze*.
 *
 * Otherwise, will look for updates for all games, respecting snooze.
 *
 * Updates found are regularly sent via @@GameUpdateAvailableNotification, and
 * then all at once in the result.
 */
export declare const CheckUpdate: import("./support").RequestCreator<CheckUpdateParams, CheckUpdateResult>;
/**
 * Result for SnoozeCave
 */
export interface SnoozeCaveResult {
}
/**
 * Snoozing a cave means we ignore all new uploads (that would
 * be potential updates) between the cave's last install operation
 * and now.
 *
 * This can be undone by calling @@CheckUpdateParams with this specific
 * cave identifier.
 */
export declare const SnoozeCave: import("./support").RequestCreator<SnoozeCaveParams, SnoozeCaveResult>;
/**
 * Result for Launch
 */
export interface LaunchResult {
}
/**
 * Attempt to launch an installed game.
 */
export declare const Launch: import("./support").RequestCreator<LaunchParams, LaunchResult>;
/**
 * Result for AcceptLicense
 */
export interface AcceptLicenseResult {
    /**
     * true if the user accepts the terms of the license, false otherwise.
     * Note that false will cancel the launch.
     */
    accept: boolean;
}
/**
 * Sent during @@LaunchParams if the game/application comes with a service license
 * agreement.
 */
export declare const AcceptLicense: import("./support").RequestCreator<AcceptLicenseParams, AcceptLicenseResult>;
/**
 * Result for PickManifestAction
 */
export interface PickManifestActionResult {
    /** Index of action picked by user, or negative if aborting */
    index: number;
}
/**
 * Sent during @@LaunchParams, ask the user to pick a manifest action to launch.
 *
 * See [itch app manifests](https://itch.io/docs/itch/integrating/manifest.html).
 */
export declare const PickManifestAction: import("./support").RequestCreator<PickManifestActionParams, PickManifestActionResult>;
/**
 * Result for ShellLaunch
 */
export interface ShellLaunchResult {
}
/**
 * Ask the client to perform a shell launch, ie. open an item
 * with the operating system's default handler (File explorer).
 *
 * Sent during @@LaunchParams.
 */
export declare const ShellLaunch: import("./support").RequestCreator<ShellLaunchParams, ShellLaunchResult>;
/**
 * Result for HTMLLaunch
 */
export interface HTMLLaunchResult {
}
/**
 * Ask the client to perform an HTML launch, ie. open an HTML5
 * game, ideally in an embedded browser.
 *
 * Sent during @@LaunchParams.
 */
export declare const HTMLLaunch: import("./support").RequestCreator<HTMLLaunchParams, HTMLLaunchResult>;
/**
 * Result for URLLaunch
 */
export interface URLLaunchResult {
}
/**
 * Ask the client to perform an URL launch, ie. open an address
 * with the system browser or appropriate.
 *
 * Sent during @@LaunchParams.
 */
export declare const URLLaunch: import("./support").RequestCreator<URLLaunchParams, URLLaunchResult>;
/**
 * Result for AllowSandboxSetup
 */
export interface AllowSandboxSetupResult {
    /** Set to true if user allowed the sandbox setup, false otherwise */
    allow: boolean;
}
/**
 * Ask the user to allow sandbox setup. Will be followed by
 * a UAC prompt (on Windows) or a pkexec dialog (on Linux) if
 * the user allows.
 *
 * Sent during @@LaunchParams.
 */
export declare const AllowSandboxSetup: import("./support").RequestCreator<AllowSandboxSetupParams, AllowSandboxSetupResult>;
/**
 * Result for PrereqsFailed
 */
export interface PrereqsFailedResult {
    /** Set to true if the user wants to proceed with the launch in spite of the prerequisites failure */
    continue: boolean;
}
/**
 * Sent during @@LaunchParams, when one or more prerequisites have failed to install.
 * The user may choose to proceed with the launch anyway.
 */
export declare const PrereqsFailed: import("./support").RequestCreator<PrereqsFailedParams, PrereqsFailedResult>;
/**
 * Result for System.StatFS
 */
export interface SystemStatFSResult {
    /** undocumented */
    freeSize: number;
    /** undocumented */
    totalSize: number;
}
/**
 * Get information on a filesystem.
 */
export declare const SystemStatFS: import("./support").RequestCreator<SystemStatFSParams, SystemStatFSResult>;
/**
 * Payload for Log
 */
export interface LogNotification {
    /** Level of the message (`info`, `warn`, etc.) */
    level: LogLevel;
    /**
     * Contents of the message.
     *
     * Note: logs may contain non-ASCII characters, or even emojis.
     */
    message: string;
}
/**
 * Sent any time butler needs to send a log message. The client should
 * relay them in their own stdout / stderr, and collect them so they
 * can be part of an issue report if something goes wrong.
 */
export declare const Log: import("./support").NotificationCreator<LogNotification>;
/**
 * undocumented
 */
export declare enum LogLevel {
    Debug = "debug",
    Info = "info",
    Warning = "warning",
    Error = "error"
}
/**
 * Result for Test.Double
 */
export interface TestDoubleResult {
    /** The number, doubled */
    number: number;
}
/**
 * Test request: return a number, doubled. Implement that to
 * use @@TestDoubleTwiceParams in your testing.
 */
export declare const TestDouble: import("./support").RequestCreator<TestDoubleParams, TestDoubleResult>;
/**
 * butlerd JSON-RPC 2.0 error codes
 */
export declare enum Code {
    OperationCancelled = 499,
    OperationAborted = 410,
    InstallFolderDisappeared = 404,
    NoCompatibleUploads = 2001,
    UnsupportedHost = 3001,
    NoLaunchCandidates = 5000,
    JavaRuntimeNeeded = 6000,
    NetworkDisconnected = 9000,
    APIError = 12000,
    DatabaseBusy = 16000,
    CantRemoveLocationBecauseOfActiveDownloads = 18000
}
/**
 * undocumented
 */
export declare type Cursor = string;
/**
 * undocumented
 */
export interface Host {
    /** os + arch, e.g. windows-i386, linux-amd64 */
    runtime: Runtime;
    /** wrapper tool (wine, etc.) that butler can launch itself */
    wrapper: Wrapper;
    /** undocumented */
    remoteLaunchName: string;
}
/**
 * undocumented
 */
export interface Wrapper {
    /** wrapper {HERE} game.exe --launch-editor */
    beforeTarget: string[];
    /** wrapper game.exe {HERE} --launch-editor */
    betweenTargetAndArgs: string[];
    /** wrapper game.exe --launch-editor {HERE} */
    afterArgs: string[];
    /** full path to the wrapper, like "wine" */
    wrapperBinary: string;
    /** additional environment variables */
    env: {
        [key: string]: string;
    };
    /**
     * When this is true, the wrapper can't function like this:
     *
     * $ wine /path/to/game.exe
     *
     * It needs to function like this:
     *
     * $ cd /path/to
     * $ wine game.exe
     *
     * This is at least true for wine, which cannot find required DLLs
     * otherwise. This might be true for other wrappers, so it's an option here.
     */
    needRelativeTarget: boolean;
}
/**
 * A Verdict contains a wealth of information on how to "launch" or "open" a specific
 * folder.
 */
export interface Verdict {
    /** BasePath is the absolute path of the folder that was configured */
    basePath: string;
    /** TotalSize is the size in bytes of the folder and all its children, recursively */
    totalSize: number;
    /** Candidates is a list of potentially interesting files, with a lot of additional info */
    candidates: Candidate[];
}
/**
 * A Candidate is a potentially interesting launch target, be it
 * a native executable, a Java or Love2D bundle, an HTML index, etc.
 */
export interface Candidate {
    /** Path is relative to the configured folder */
    path: string;
    /** Mode describes file permissions */
    mode: number;
    /** Depth is the number of path elements leading up to this candidate */
    depth: number;
    /** Flavor is the type of a candidate - native, html, jar etc. */
    flavor: Flavor;
    /** Arch describes the architecture of a candidate (where relevant) */
    arch: Arch;
    /** Size is the size of the candidate's file, in bytes */
    size: number;
    /** Spell contains raw output from <https://github.com/itchio/wizardry> */
    spell?: string[];
    /** WindowsInfo contains information specific to native Windows candidates */
    windowsInfo?: WindowsInfo;
    /** LinuxInfo contains information specific to native Linux candidates */
    linuxInfo?: LinuxInfo;
    /** MacosInfo contains information specific to native macOS candidates */
    macosInfo?: MacosInfo;
    /** LoveInfo contains information specific to Love2D bundles (`.love` files) */
    loveInfo?: LoveInfo;
    /** ScriptInfo contains information specific to shell scripts (`.sh`, `.bat` etc.) */
    scriptInfo?: ScriptInfo;
    /** JarInfo contains information specific to Java archives (`.jar` files) */
    jarInfo?: JarInfo;
}
/**
 * Flavor describes whether we're dealing with a native executables, a Java archive, a love2d bundle, etc.
 */
export declare enum Flavor {
    NativeLinux = "linux",
    NativeMacos = "macos",
    NativeWindows = "windows",
    AppMacos = "app-macos",
    Script = "script",
    ScriptWindows = "windows-script",
    Jar = "jar",
    HTML = "html",
    Love = "love",
    MSI = "msi"
}
/**
 * The architecture of an executable
 */
export declare enum Arch {
    _386 = "386",
    Amd64 = "amd64"
}
/**
 * Contains information specific to native windows executables
 * or installer packages.
 */
export interface WindowsInfo {
    /** Particular type of installer (msi, inno, etc.) */
    installerType?: WindowsInstallerType;
    /** True if we suspect this might be an uninstaller rather than an installer */
    uninstaller?: boolean;
    /** Is this executable marked as GUI? This can be false and still pop a GUI, it's just a hint. */
    gui?: boolean;
    /** Is this a .NET assembly? */
    dotNet?: boolean;
}
/**
 * Which particular type of windows-specific installer
 */
export declare enum WindowsInstallerType {
    Msi = "msi",
    Inno = "inno",
    Nullsoft = "nsis",
    Archive = "archive"
}
/**
 * Contains information specific to native macOS executables
 * or app bundles.
 */
export interface MacosInfo {
}
/**
 * Contains information specific to native Linux executables
 */
export interface LinuxInfo {
}
/**
 * Contains information specific to Love2D bundles
 */
export interface LoveInfo {
    /** The version of love2D required to open this bundle. May be empty */
    version?: string;
}
/**
 * Contains information specific to shell scripts
 */
export interface ScriptInfo {
    /** Something like `/bin/bash` */
    interpreter?: string;
}
/**
 * Contains information specific to Java archives
 */
export interface JarInfo {
    /** The main Java class as specified by the manifest included in the .jar (if any) */
    mainClass?: string;
}
/**
 * User represents an itch.io account, with basic profile info
 */
export interface User {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** The user's username (used for login) */
    username: string;
    /** The user's display name: human-friendly, may contain spaces, unicode etc. */
    displayName: string;
    /** Has the user opted into creating games? */
    developer: boolean;
    /** Is the user part of itch.io's press program? */
    pressUser: boolean;
    /** The address of the user's page on itch.io */
    url: string;
    /** User's avatar, may be a GIF */
    coverUrl: string;
    /** Static version of user's avatar, only set if the main cover URL is a GIF */
    stillCoverUrl: string;
}
/**
 * Game represents a page on itch.io, it could be a game,
 * a tool, a comic, etc.
 */
export interface Game {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Canonical address of the game's page on itch.io */
    url: string;
    /** Human-friendly title (may contain any character) */
    title: string;
    /** Human-friendly short description */
    shortText: string;
    /** Downloadable game, html game, etc. */
    type: GameType;
    /** Classification: game, tool, comic, etc. */
    classification: GameClassification;
    /** Configuration for embedded (HTML5) games */
    embed?: GameEmbedData;
    /** Cover url (might be a GIF) */
    coverUrl: string;
    /** Non-gif cover url, only set if main cover url is a GIF */
    stillCoverUrl: string;
    /** Date the game was created */
    createdAt: RFCDate;
    /** Date the game was published, empty if not currently published */
    publishedAt: RFCDate;
    /** Price in cents of a dollar */
    minPrice: number;
    /** Are payments accepted? */
    canBeBought: boolean;
    /** Does this game have a demo available? */
    hasDemo: boolean;
    /** Is this game part of the itch.io press system? */
    inPressSystem: boolean;
    /** Platforms this game is available for */
    platforms: Platforms;
    /** The user account this game is associated to */
    user?: User;
    /** ID of the user account this game is associated to */
    userId: number;
    /** The best current sale for this game */
    sale?: Sale;
    /** undocumented */
    viewsCount: number;
    /** undocumented */
    downloadsCount: number;
    /** undocumented */
    purchasesCount: number;
    /** undocumented */
    published: boolean;
}
/**
 * Platforms describes which OS/architectures a game or upload
 * is compatible with.
 */
export interface Platforms {
    /** undocumented */
    windows: Architectures;
    /** undocumented */
    linux: Architectures;
    /** undocumented */
    osx: Architectures;
}
/**
 * Architectures describes a set of processor architectures (mostly 32-bit vs 64-bit)
 */
export declare enum Architectures {
    All = "all",
    _386 = "386",
    Amd64 = "amd64"
}
/**
 * GameType is the type of an itch.io game page, mostly related to
 * how it should be presented on web (downloadable or embed)
 */
export declare enum GameType {
    Default = "default",
    Flash = "flash",
    Unity = "unity",
    Java = "java",
    HTML = "html"
}
/**
 * GameClassification is the creator-picked classification for a page
 */
export declare enum GameClassification {
    Game = "game",
    Tool = "tool",
    Assets = "assets",
    GameMod = "game_mod",
    PhysicalGame = "physical_game",
    Soundtrack = "soundtrack",
    Other = "other",
    Comic = "comic",
    Book = "book"
}
/**
 * GameEmbedData contains presentation information for embed games
 */
export interface GameEmbedData {
    /** Game this embed info is for */
    gameId: number;
    /** width of the initial viewport, in pixels */
    width: number;
    /** height of the initial viewport, in pixels */
    height: number;
    /** for itch.io website, whether or not a fullscreen button should be shown */
    fullscreen: boolean;
}
/**
 * Sale describes a discount for a game.
 */
export interface Sale {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Game this sale is for */
    gameId: number;
    /**
     * Discount rate in percent.
     * Can be negative, see https://itch.io/updates/introducing-reverse-sales
     */
    rate: number;
    /** Timestamp the sale started at */
    startDate: RFCDate;
    /** Timestamp the sale ends at */
    endDate: RFCDate;
}
/**
 * An Upload is a downloadable file. Some are wharf-enabled, which means
 * they're actually a "channel" that may contain multiple builds, pushed
 * with <https://github.com/itchio/butler>
 */
export interface Upload {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Storage (hosted, external, etc.) */
    storage: UploadStorage;
    /** Host (if external storage) */
    host: string;
    /** Original file name (example: `Overland_x64.zip`) */
    filename: string;
    /** Human-friendly name set by developer (example: `Overland for Windows 64-bit`) */
    displayName: string;
    /** Size of upload in bytes. For wharf-enabled uploads, it's the archive size. */
    size: number;
    /** Name of the wharf channel for this upload, if it's a wharf-enabled upload */
    channelName: string;
    /** Latest build for this upload, if it's a wharf-enabled upload */
    build: Build;
    /** ID of the latest build for this upload, if it's a wharf-enabled upload */
    buildId: number;
    /** Upload type: default, soundtrack, etc. */
    type: UploadType;
    /** Is this upload a pre-order placeholder? */
    preorder: boolean;
    /** Is this upload a free demo? */
    demo: boolean;
    /** Platforms this upload is compatible with */
    platforms: Platforms;
    /** Date this upload was created at */
    createdAt: RFCDate;
    /** Date this upload was last updated at (order changed, display name set, etc.) */
    updatedAt: RFCDate;
}
/**
 * UploadStorage describes where an upload file is stored.
 */
export declare enum UploadStorage {
    Hosted = "hosted",
    Build = "build",
    External = "external"
}
/**
 * UploadType describes what's in an upload - an executable,
 * a web game, some music, etc.
 */
export declare enum UploadType {
    Default = "default",
    Flash = "flash",
    Unity = "unity",
    Java = "java",
    HTML = "html",
    Soundtrack = "soundtrack",
    Book = "book",
    Video = "video",
    Documentation = "documentation",
    Mod = "mod",
    AudioAssets = "audio_assets",
    GraphicalAssets = "graphical_assets",
    Sourcecode = "sourcecode",
    Other = "other"
}
/**
 * A Collection is a set of games, curated by humans.
 */
export interface Collection {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Human-friendly title for collection, for example `Couch coop games` */
    title: string;
    /** Date this collection was created at */
    createdAt: RFCDate;
    /** Date this collection was last updated at (item added, title set, etc.) */
    updatedAt: RFCDate;
    /**
     * Number of games in the collection. This might not be accurate
     * as some games might not be accessible to whoever is asking (project
     * page deleted, visibility level changed, etc.)
     */
    gamesCount: number;
    /** Games in this collection, with additional info */
    collectionGames: CollectionGame[];
    /** undocumented */
    userId: number;
    /** undocumented */
    user: User;
}
/**
 * CollectionGame represents a game's membership for a collection.
 */
export interface CollectionGame {
    /** undocumented */
    collectionId: number;
    /** undocumented */
    collection: Collection;
    /** undocumented */
    gameId: number;
    /** undocumented */
    game: Game;
    /** undocumented */
    position: number;
    /** undocumented */
    createdAt: RFCDate;
    /** undocumented */
    updatedAt: RFCDate;
    /** undocumented */
    blurb: string;
    /** undocumented */
    userId: number;
}
/**
 * A DownloadKey is often generated when a purchase is made, it
 * allows downloading uploads for a game that are not available
 * for free. It can also be generated by other means.
 */
export interface DownloadKey {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Identifier of the game to which this download key grants access */
    gameId: number;
    /** Game to which this download key grants access */
    game: Game;
    /** Date this key was created at (often coincides with purchase time) */
    createdAt: RFCDate;
    /** Date this key was last updated at */
    updatedAt: RFCDate;
    /** Identifier of the itch.io user to which this key belongs */
    ownerId: number;
}
/**
 * Build contains information about a specific build
 */
export interface Build {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /**
     * Identifier of the build before this one on the same channel,
     * or 0 if this is the initial build.
     */
    parentBuildId: number;
    /** State of the build: started, processing, etc. */
    state: BuildState;
    /** Automatically-incremented version number, starting with 1 */
    version: number;
    /**
     * Value specified by developer with `--userversion` when pushing a build
     * Might not be unique across builds of a given channel.
     */
    userVersion: string;
    /**
     * Files associated with this build - often at least an archive,
     * a signature, and a patch. Some might be missing while the build
     * is still processing or if processing has failed.
     */
    files: BuildFile[];
    /** User who pushed the build */
    user: User;
    /** Timestamp the build was created at */
    createdAt: RFCDate;
    /** Timestamp the build was last updated at */
    updatedAt: RFCDate;
}
/**
 * BuildState describes the state of a build, relative to its initial upload, and
 * its processing.
 */
export declare enum BuildState {
    Started = "started",
    Processing = "processing",
    Completed = "completed",
    Failed = "failed"
}
/**
 * BuildFile contains information about a build's "file", which could be its
 * archive, its signature, its patch, etc.
 */
export interface BuildFile {
    /** Site-wide unique identifier generated by itch.io */
    id: number;
    /** Size of this build file */
    size: number;
    /** State of this file: created, uploading, uploaded, etc. */
    state: BuildFileState;
    /** Type of this build file: archive, signature, patch, etc. */
    type: BuildFileType;
    /** Subtype of this build file, usually indicates compression */
    subType: BuildFileSubType;
    /** Date this build file was created at */
    createdAt: RFCDate;
    /** Date this build file was last updated at */
    updatedAt: RFCDate;
}
/**
 * BuildFileState describes the state of a specific file for a build
 */
export declare enum BuildFileState {
    Created = "created",
    Uploading = "uploading",
    Uploaded = "uploaded",
    Failed = "failed"
}
/**
 * BuildFileType describes the type of a build file: patch, archive, signature, etc.
 */
export declare enum BuildFileType {
    Patch = "patch",
    Archive = "archive",
    Signature = "signature",
    Manifest = "manifest",
    Unpacked = "unpacked"
}
/**
 * BuildFileSubType describes the subtype of a build file: mostly its compression
 * level. For example, rediff'd patches are "optimized", whereas initial patches are "default"
 */
export declare enum BuildFileSubType {
    Default = "default",
    Gzip = "gzip",
    Optimized = "optimized"
}
/**
 * undocumented
 */
export interface InstallEvent {
    /** undocumented */
    type: InstallEventType;
    /** undocumented */
    timestamp: RFCDate;
    /** undocumented */
    heal: HealInstallEvent;
    /** undocumented */
    install: InstallInstallEvent;
    /** undocumented */
    upgrade: UpgradeInstallEvent;
    /** undocumented */
    ghostBusting: GhostBustingInstallEvent;
    /** undocumented */
    patching: PatchingInstallEvent;
    /** undocumented */
    problem: ProblemInstallEvent;
    /** undocumented */
    fallback: FallbackInstallEvent;
}
/**
 * undocumented
 */
export declare enum InstallEventType {
    InstallEventResume = "resume",
    InstallEventStop = "stop",
    InstallEventInstall = "install",
    InstallEventHeal = "heal",
    InstallEventUpgrade = "upgrade",
    InstallEventPatching = "patching",
    InstallEventGhostBusting = "ghostBusting",
    InstallEventProblem = "problem",
    InstallEventFallback = "fallback"
}
/**
 * undocumented
 */
export interface InstallInstallEvent {
    /** undocumented */
    manager: string;
}
/**
 * undocumented
 */
export interface HealInstallEvent {
    /** undocumented */
    totalCorrupted: number;
    /** undocumented */
    appliedCaseFixes: boolean;
}
/**
 * undocumented
 */
export interface UpgradeInstallEvent {
    /** undocumented */
    numPatches: number;
}
/**
 * undocumented
 */
export interface ProblemInstallEvent {
    /** Short error */
    error: string;
    /** Longer error */
    errorStack: string;
}
/**
 * undocumented
 */
export interface FallbackInstallEvent {
    /** Name of the operation we were trying to do */
    attempted: string;
    /** Problem encountered while trying "attempted" */
    problem: ProblemInstallEvent;
    /** Name of the operation we're falling back to */
    nowTrying: string;
}
/**
 * undocumented
 */
export interface PatchingInstallEvent {
    /** Build we patched to */
    buildID: number;
    /** "default" or "optimized" (for the +bsdiff variant) */
    subtype: string;
}
/**
 * undocumented
 */
export interface GhostBustingInstallEvent {
    /** Operation that requested the ghost busting (install, upgrade, heal) */
    operation: string;
    /** Number of ghost files found */
    found: number;
    /** Number of ghost files removed */
    removed: number;
}
/**
 * A Receipt describes what was installed to a specific folder.
 *
 * It's compressed and written to `./.itch/receipt.json.gz` every
 * time an install operation completes successfully, and is used
 * in further install operations to make sure ghosts are busted and/or
 * angels are saved.
 */
export interface Receipt {
    /** The itch.io game installed at this location */
    game: Game;
    /** The itch.io upload installed at this location */
    upload: Upload;
    /** The itch.io build installed at this location. Null for non-wharf upload. */
    build: Build;
    /** A list of installed files (slash-separated paths, relative to install folder) */
    files: string[];
    /** The installer used to install at this location */
    installerName?: string;
}
/**
 * A Manifest describes prerequisites (dependencies) and actions that
 * can be taken while launching a game.
 */
export interface Manifest {
    /** Actions are a list of options to give the user when launching a game. */
    actions: Actions;
    /**
     * Prereqs describe libraries or frameworks that must be installed
     * prior to launching a game
     */
    prereqs: Prereq[];
}
/**
 * undocumented
 */
export declare type Actions = Action[];
/**
 * An Action is a choice for the user to pick when launching a game.
 *
 * see https://itch.io/docs/itch/integrating/manifest.html
 */
export interface Action {
    /** human-readable or standard name */
    name: string;
    /** file path (relative to manifest or absolute), URL, etc. */
    path: string;
    /** icon name (see static/fonts/icomoon/demo.html, don't include `icon-` prefix) */
    icon: string;
    /** command-line arguments */
    args: string[];
    /** sandbox opt-in */
    sandbox: boolean;
    /** requested API scope */
    scope: string;
    /** don't redirect stdout/stderr, open in new console window */
    console: boolean;
    /** platform to restrict this action to */
    platform: Platform;
    /** localized action name */
    locales: {
        [key: string]: ActionLocale;
    };
}
/**
 * undocumented
 */
export interface Prereq {
    /** A prerequisite to be installed, see <https://itch.io/docs/itch/integrating/prereqs/> for the full list. */
    name: string;
}
/**
 * undocumented
 */
export interface ActionLocale {
    /** A localized action name */
    name: string;
}
/**
 * undocumented
 */
export declare enum Platform {
    OSX = "osx",
    Windows = "windows",
    Linux = "linux",
    Unknown = "unknown"
}
/**
 * Runtime describes an os-arch combo in a convenient way
 */
export interface Runtime {
    /** undocumented */
    platform: Platform;
    /** undocumented */
    is64: boolean;
}
/**
 * undocumented
 */
export declare type Runtimes = Runtime[];
/**
 * Params for Meta.Authenticate
 */
export interface MetaAuthenticateParams {
    /** undocumented */
    secret: string;
}
/**
 * Params for Meta.Flow
 */
export interface MetaFlowParams {
}
/**
 * Params for Meta.Shutdown
 */
export interface MetaShutdownParams {
}
/**
 * Payload for MetaFlowEstablished
 */
export interface MetaFlowEstablishedNotification {
    /** The identifier of the daemon process for which the flow was established */
    pid: number;
}
/**
 * The first notification sent when @@MetaFlowParams is called.
 */
export declare const MetaFlowEstablished: import("./support").NotificationCreator<MetaFlowEstablishedNotification>;
/**
 * Params for Version.Get
 */
export interface VersionGetParams {
}
/**
 * Params for Network.SetSimulateOffline
 */
export interface NetworkSetSimulateOfflineParams {
    /**
     * If true, all operations after this point will behave
     * as if there were no network connections
     */
    enabled: boolean;
}
/**
 * Params for Network.SetBandwidthThrottle
 */
export interface NetworkSetBandwidthThrottleParams {
    /** If true, will limit. If false, will clear any bandwidth throttles in place */
    enabled: boolean;
    /** The target bandwidth, in kbps */
    rate: number;
}
/**
 * Params for Profile.List
 */
export interface ProfileListParams {
}
/**
 * Params for Profile.LoginWithPassword
 */
export interface ProfileLoginWithPasswordParams {
    /** The username (or e-mail) to use for login */
    username: string;
    /** The password to use */
    password: string;
}
/**
 * Params for Profile.LoginWithAPIKey
 */
export interface ProfileLoginWithAPIKeyParams {
    /** The API token to use */
    apiKey: string;
}
/**
 * Params for Profile.RequestCaptcha
 */
export interface ProfileRequestCaptchaParams {
    /** Address of page containing a recaptcha widget */
    recaptchaUrl: string;
}
/**
 * Params for Profile.RequestTOTP
 */
export interface ProfileRequestTOTPParams {
}
/**
 * Params for Profile.UseSavedLogin
 */
export interface ProfileUseSavedLoginParams {
    /** undocumented */
    profileId: number;
}
/**
 * Params for Profile.Forget
 */
export interface ProfileForgetParams {
    /** undocumented */
    profileId: number;
}
/**
 * Params for Profile.Data.Put
 */
export interface ProfileDataPutParams {
    /** undocumented */
    profileId: number;
    /** undocumented */
    key: string;
    /** undocumented */
    value: string;
}
/**
 * Params for Profile.Data.Get
 */
export interface ProfileDataGetParams {
    /** undocumented */
    profileId: number;
    /** undocumented */
    key: string;
}
/**
 * Params for Search.Games
 */
export interface SearchGamesParams {
    /** undocumented */
    profileId: number;
    /** undocumented */
    query: string;
}
/**
 * Params for Search.Users
 */
export interface SearchUsersParams {
    /** undocumented */
    profileId: number;
    /** undocumented */
    query: string;
}
/**
 * Params for Fetch.Game
 */
export interface FetchGameParams {
    /** Identifier of game to look for */
    gameId: number;
    /** Force an API request */
    fresh?: boolean;
}
/**
 * Params for Fetch.GameRecords
 */
export interface FetchGameRecordsParams {
    /** Profile to use to fetch game */
    profileId: number;
    /** Source from which to fetch games */
    source: GameRecordsSource;
    /** Collection ID, required if `Source` is "collection" */
    collectionId?: number;
    /** Maximum number of games to return at a time */
    limit?: number;
    /** Games to skip */
    offset?: number;
    /** When specified only shows game titles that contain this string */
    search?: string;
    /** Criterion to sort by */
    sortBy?: string;
    /** Filters */
    filters?: GameRecordsFilters;
    /** undocumented */
    reverse?: boolean;
    /** If set, will force fresh data */
    fresh?: boolean;
}
/**
 * Params for Fetch.DownloadKey
 */
export interface FetchDownloadKeyParams {
    /** undocumented */
    downloadKeyId: number;
    /** undocumented */
    profileId: number;
    /** Force an API request */
    fresh?: boolean;
}
/**
 * Params for Fetch.DownloadKeys
 */
export interface FetchDownloadKeysParams {
    /** undocumented */
    profileId: number;
    /** Number of items to skip */
    offset?: number;
    /** Max number of results per page (default = 5) */
    limit?: number;
    /** Filter results */
    filters?: FetchDownloadKeysFilter;
    /** Force an API request */
    fresh?: boolean;
}
/**
 * Params for Fetch.GameUploads
 */
export interface FetchGameUploadsParams {
    /** Identifier of the game whose uploads we should look for */
    gameId: number;
    /** Only returns compatible uploads */
    compatible: boolean;
    /** Force an API request */
    fresh?: boolean;
}
/**
 * Params for Fetch.User
 */
export interface FetchUserParams {
    /** Identifier of the user to look for */
    userId: number;
    /** Profile to use to look upser */
    profileId: number;
    /** Force an API request */
    fresh?: boolean;
}
/**
 * Params for Fetch.Sale
 */
export interface FetchSaleParams {
    /** Identifier of the game for which to look for a sale */
    gameId: number;
}
/**
 * Params for Fetch.Collection
 */
export interface FetchCollectionParams {
    /** Profile to use to fetch collection */
    profileId: number;
    /** Collection to fetch */
    collectionId: number;
    /**
     * Force an API request before replying.
     * Usually set after getting 'stale' in the response.
     */
    fresh?: boolean;
}
/**
 * Params for Fetch.Collection.Games
 */
export interface FetchCollectionGamesParams {
    /** Profile to use to fetch collection */
    profileId: number;
    /** Identifier of the collection to look for */
    collectionId: number;
    /** Maximum number of games to return at a time. */
    limit?: number;
    /** When specified only shows game titles that contain this string */
    search?: string;
    /** Criterion to sort by */
    sortBy?: string;
    /** Filters */
    filters?: CollectionGamesFilters;
    /** undocumented */
    reverse?: boolean;
    /** Used for pagination, if specified */
    cursor?: Cursor;
    /** If set, will force fresh data */
    fresh?: boolean;
}
/**
 * Params for Fetch.ProfileCollections
 */
export interface FetchProfileCollectionsParams {
    /** Profile for which to fetch collections */
    profileId: number;
    /** Maximum number of collections to return at a time. */
    limit?: number;
    /** When specified only shows collection titles that contain this string */
    search?: string;
    /** Criterion to sort by */
    sortBy?: string;
    /** undocumented */
    reverse?: boolean;
    /** Used for pagination, if specified */
    cursor?: Cursor;
    /** If set, will force fresh data */
    fresh?: boolean;
}
/**
 * Params for Fetch.ProfileGames
 */
export interface FetchProfileGamesParams {
    /** Profile for which to fetch games */
    profileId: number;
    /** Maximum number of items to return at a time. */
    limit?: number;
    /** When specified only shows game titles that contain this string */
    search?: string;
    /** Criterion to sort by */
    sortBy?: string;
    /** Filters */
    filters?: ProfileGameFilters;
    /** undocumented */
    reverse?: boolean;
    /** Used for pagination, if specified */
    cursor?: Cursor;
    /** If set, will force fresh data */
    fresh?: boolean;
}
/**
 * Params for Fetch.ProfileOwnedKeys
 */
export interface FetchProfileOwnedKeysParams {
    /** Profile to use to fetch game */
    profileId: number;
    /** Maximum number of owned keys to return at a time. */
    limit?: number;
    /** When specified only shows game titles that contain this string */
    search?: string;
    /** Criterion to sort by */
    sortBy?: string;
    /** Filters */
    filters?: ProfileOwnedKeysFilters;
    /** undocumented */
    reverse?: boolean;
    /** Used for pagination, if specified */
    cursor?: Cursor;
    /** If set, will force fresh data */
    fresh?: boolean;
}
/**
 * Params for Fetch.Commons
 */
export interface FetchCommonsParams {
}
/**
 * Params for Fetch.Caves
 */
export interface FetchCavesParams {
    /** Maximum number of caves to return at a time. */
    limit?: number;
    /** When specified only shows game titles that contain this string */
    search?: string;
    /** undocumented */
    sortBy?: string;
    /** Filters */
    filters?: CavesFilters;
    /** undocumented */
    reverse?: boolean;
    /** Used for pagination, if specified */
    cursor?: Cursor;
}
/**
 * Params for Fetch.Cave
 */
export interface FetchCaveParams {
    /** undocumented */
    caveId: string;
}
/**
 * Params for Fetch.ExpireAll
 */
export interface FetchExpireAllParams {
}
/**
 * Params for Game.FindUploads
 */
export interface GameFindUploadsParams {
    /** Which game to find uploads for */
    game: Game;
}
/**
 * Params for Install.Queue
 */
export interface InstallQueueParams {
    /**
     * ID of the cave to perform the install for.
     * If not specified, will create a new cave.
     */
    caveId?: string;
    /** If unspecified, will default to 'install' */
    reason?: DownloadReason;
    /**
     * If CaveID is not specified, ID of an install location
     * to install to.
     */
    installLocationId?: string;
    /**
     * If set, InstallFolder can be set and no cave
     * record will be read or modified
     */
    noCave?: boolean;
    /** When NoCave is set, exactly where to install */
    installFolder?: string;
    /**
     * Which game to install.
     *
     * If unspecified and caveId is specified, the same game will be used.
     */
    game?: Game;
    /**
     * Which upload to install.
     *
     * If unspecified and caveId is specified, the same upload will be used.
     */
    upload?: Upload;
    /**
     * Which build to install
     *
     * If unspecified and caveId is specified, the same build will be used.
     */
    build?: Build;
    /**
     * If true, do not run windows installers, just extract
     * whatever to the install folder.
     */
    ignoreInstallers?: boolean;
    /**
     * A folder that butler can use to store temporary files, like
     * partial downloads, checkpoint files, etc.
     */
    stagingFolder?: string;
    /**
     * If set, and the install operation is successfully disambiguated,
     * will queue it as a download for butler to drive.
     * See @@DownloadsDriveParams.
     */
    queueDownload?: boolean;
    /** Don't run install prepare (assume we can just run it at perform time) */
    fastQueue?: boolean;
}
/**
 * Params for Install.Plan
 */
export interface InstallPlanParams {
    /** The ID of the game we're planning to install */
    gameId: number;
    /** The download session ID to use for this install plan */
    downloadSessionId?: string;
    /** undocumented */
    uploadId?: number;
}
/**
 * Params for Caves.SetPinned
 */
export interface CavesSetPinnedParams {
    /** ID of the cave to pin/unpin */
    caveId: string;
    /** Pinned state the cave should have after this call */
    pinned: boolean;
}
/**
 * Params for Install.CreateShortcut
 */
export interface InstallCreateShortcutParams {
    /** undocumented */
    caveId: string;
}
/**
 * Params for Install.Perform
 */
export interface InstallPerformParams {
    /** ID that can be later used in @@InstallCancelParams */
    id: string;
    /** The folder turned by @@InstallQueueParams */
    stagingFolder: string;
}
/**
 * Params for Install.Cancel
 */
export interface InstallCancelParams {
    /** The UUID of the task to cancel, as passed to @@OperationStartParams */
    id: string;
}
/**
 * Params for Uninstall.Perform
 */
export interface UninstallPerformParams {
    /** The cave to uninstall */
    caveId: string;
    /**
     * If true, don't attempt to run any uninstallers, just
     * remove the DB record and burn the install folder to the ground.
     */
    hard?: boolean;
}
/**
 * Params for Install.VersionSwitch.Queue
 */
export interface InstallVersionSwitchQueueParams {
    /** The cave to switch to a different version */
    caveId: string;
}
/**
 * Params for InstallVersionSwitchPick
 */
export interface InstallVersionSwitchPickParams {
    /** undocumented */
    cave: Cave;
    /** undocumented */
    upload: Upload;
    /** undocumented */
    builds: Build[];
}
/**
 * Params for PickUpload
 */
export interface PickUploadParams {
    /** An array of upload objects to choose from */
    uploads: Upload[];
}
/**
 * Payload for Progress
 */
export interface ProgressNotification {
    /** An overall progress value between 0 and 1 */
    progress: number;
    /** Estimated completion time for the operation, in seconds (floating) */
    eta: number;
    /** Network bandwidth used, in bytes per second (floating) */
    bps: number;
}
/**
 * Sent periodically during @@InstallPerformParams to inform on the current state of an install
 */
export declare const Progress: import("./support").NotificationCreator<ProgressNotification>;
/**
 * undocumented
 */
export declare enum TaskReason {
    Install = "install",
    Uninstall = "uninstall"
}
/**
 * undocumented
 */
export declare enum TaskType {
    Download = "download",
    Install = "install",
    Uninstall = "uninstall",
    Update = "update",
    Heal = "heal"
}
/**
 * Payload for TaskStarted
 */
export interface TaskStartedNotification {
    /** Why this task was started */
    reason: TaskReason;
    /** Is this task a download? An install? */
    type: TaskType;
    /** The game this task is dealing with */
    game: Game;
    /** The upload this task is dealing with */
    upload: Upload;
    /** The build this task is dealing with (if any) */
    build: Build;
    /** Total size in bytes */
    totalSize: number;
}
/**
 * Each operation is made up of one or more tasks. This notification
 * is sent during @@OperationStartParams whenever a specific task starts.
 */
export declare const TaskStarted: import("./support").NotificationCreator<TaskStartedNotification>;
/**
 * Payload for TaskSucceeded
 */
export interface TaskSucceededNotification {
    /** undocumented */
    type: TaskType;
    /**
     * If the task installed something, then this contains
     * info about the game, upload, build that were installed
     */
    installResult: InstallResult;
}
/**
 * Sent during @@OperationStartParams whenever a task succeeds for an operation.
 */
export declare const TaskSucceeded: import("./support").NotificationCreator<TaskSucceededNotification>;
/**
 * What was installed by a subtask of @@OperationStartParams.
 *
 * See @@TaskSucceededNotification.
 */
export interface InstallResult {
    /** The game we installed */
    game: Game;
    /** The upload we installed */
    upload: Upload;
    /** The build we installed */
    build?: Build;
}
/**
 * Params for Install.Locations.List
 */
export interface InstallLocationsListParams {
}
/**
 * Params for Install.Locations.Add
 */
export interface InstallLocationsAddParams {
    /**
     * identifier of the new install location.
     * if not specified, will be generated.
     */
    id?: string;
    /** path of the new install location */
    path: string;
}
/**
 * Params for Install.Locations.Remove
 */
export interface InstallLocationsRemoveParams {
    /** identifier of the install location to remove */
    id: string;
}
/**
 * Params for Install.Locations.GetByID
 */
export interface InstallLocationsGetByIDParams {
    /** identifier of the install location to remove */
    id: string;
}
/**
 * Params for Install.Locations.Scan
 */
export interface InstallLocationsScanParams {
    /** path to a legacy marketDB */
    legacyMarketPath?: string;
}
/**
 * Payload for Install.Locations.Scan.Yield
 */
export interface InstallLocationsScanYieldNotification {
    /** undocumented */
    game: Game;
}
/**
 * Sent during @@InstallLocationsScanParams whenever
 * a game is found.
 */
export declare const InstallLocationsScanYield: import("./support").NotificationCreator<InstallLocationsScanYieldNotification>;
/**
 * Params for Install.Locations.Scan.ConfirmImport
 */
export interface InstallLocationsScanConfirmImportParams {
    /** number of items that will be imported */
    numItems: number;
}
/**
 * Params for Downloads.Queue
 */
export interface DownloadsQueueParams {
    /** undocumented */
    item: InstallQueueResult;
}
/**
 * Params for Downloads.Prioritize
 */
export interface DownloadsPrioritizeParams {
    /** undocumented */
    downloadId: string;
}
/**
 * Params for Downloads.List
 */
export interface DownloadsListParams {
}
/**
 * Params for Downloads.ClearFinished
 */
export interface DownloadsClearFinishedParams {
}
/**
 * Params for Downloads.Drive
 */
export interface DownloadsDriveParams {
}
/**
 * Params for Downloads.Drive.Cancel
 */
export interface DownloadsDriveCancelParams {
}
/**
 * Params for Downloads.Retry
 */
export interface DownloadsRetryParams {
    /** undocumented */
    downloadId: string;
}
/**
 * Params for Downloads.Discard
 */
export interface DownloadsDiscardParams {
    /** undocumented */
    downloadId: string;
}
/**
 * Params for CheckUpdate
 */
export interface CheckUpdateParams {
    /** If specified, will only look for updates to these caves */
    caveIds?: string[];
    /** If specified, will log information even when we have no warnings/errors */
    verbose?: boolean;
}
/**
 * Payload for GameUpdateAvailable
 */
export interface GameUpdateAvailableNotification {
    /** undocumented */
    update: GameUpdate;
}
/**
 * Sent during @@CheckUpdateParams, every time butler
 * finds an update for a game. Can be safely ignored if displaying
 * updates as they are found is not a requirement for the client.
 */
export declare const GameUpdateAvailable: import("./support").NotificationCreator<GameUpdateAvailableNotification>;
/**
 * Describes an available update for a particular game install.
 */
export interface GameUpdate {
    /** Cave we found an update for */
    caveId: string;
    /** Game we found an update for */
    game: Game;
    /**
     * True if this is a direct update, ie. we're on
     * a channel that still exists, and there's a new build
     * False if it's an indirect update, for example a new
     * upload that appeared after we installed, but we're
     * not sure if it's an upgrade or other additional content
     */
    direct: boolean;
    /** Available choice of updates */
    choices: GameUpdateChoice[];
}
/**
 * Params for SnoozeCave
 */
export interface SnoozeCaveParams {
    /** undocumented */
    caveId: string;
}
/**
 * One possible upload/build choice to upgrade a cave
 */
export interface GameUpdateChoice {
    /** Upload to be installed */
    upload: Upload;
    /** Build to be installed (may be nil) */
    build: Build;
    /** How confident we are that this is the right upgrade */
    confidence: number;
}
/**
 * Params for Launch
 */
export interface LaunchParams {
    /** The ID of the cave to launch */
    caveId: string;
    /** The directory to use to store installer files for prerequisites */
    prereqsDir: string;
    /** Force installing all prerequisites, even if they're already marked as installed */
    forcePrereqs?: boolean;
    /** Enable sandbox (regardless of manifest opt-in) */
    sandbox?: boolean;
}
/**
 * Payload for LaunchRunning
 */
export interface LaunchRunningNotification {
}
/**
 * Sent during @@LaunchParams, when the game is configured, prerequisites are installed
 * sandbox is set up (if enabled), and the game is actually running.
 */
export declare const LaunchRunning: import("./support").NotificationCreator<LaunchRunningNotification>;
/**
 * Payload for LaunchExited
 */
export interface LaunchExitedNotification {
}
/**
 * Sent during @@LaunchParams, when the game has actually exited.
 */
export declare const LaunchExited: import("./support").NotificationCreator<LaunchExitedNotification>;
/**
 * Params for AcceptLicense
 */
export interface AcceptLicenseParams {
    /**
     * The full text of the license agreement, in its default
     * language, which is usually English.
     */
    text: string;
}
/**
 * Params for PickManifestAction
 */
export interface PickManifestActionParams {
    /** A list of actions to pick from. Must be shown to the user in the order they're passed. */
    actions: Action[];
}
/**
 * Params for ShellLaunch
 */
export interface ShellLaunchParams {
    /** Absolute path of item to open, e.g. `D:\\Games\\Itch\\garden\\README.txt` */
    itemPath: string;
}
/**
 * Params for HTMLLaunch
 */
export interface HTMLLaunchParams {
    /** Absolute path on disk to serve */
    rootFolder: string;
    /** Path of index file, relative to root folder */
    indexPath: string;
    /** Command-line arguments, to pass as `global.Itch.args` */
    args: string[];
    /** Environment variables, to pass as `global.Itch.env` */
    env: {
        [key: string]: string;
    };
}
/**
 * Params for URLLaunch
 */
export interface URLLaunchParams {
    /** URL to open, e.g. `https://itch.io/community` */
    url: string;
}
/**
 * Params for AllowSandboxSetup
 */
export interface AllowSandboxSetupParams {
}
/**
 * Payload for PrereqsStarted
 */
export interface PrereqsStartedNotification {
    /** A list of prereqs that need to be tended to */
    tasks: {
        [key: string]: PrereqTask;
    };
}
/**
 * Sent during @@LaunchParams, when some prerequisites are about to be installed.
 *
 * This is a good time to start showing a UI element with the state of prereq
 * tasks.
 *
 * Updates are regularly provided via @@PrereqsTaskStateNotification.
 */
export declare const PrereqsStarted: import("./support").NotificationCreator<PrereqsStartedNotification>;
/**
 * Information about a prerequisite task.
 */
export interface PrereqTask {
    /** Full name of the prerequisite, for example: `Microsoft .NET Framework 4.6.2` */
    fullName: string;
    /** Order of task in the list. Respect this order in the UI if you want consistent progress indicators. */
    order: number;
}
/**
 * Payload for PrereqsTaskState
 */
export interface PrereqsTaskStateNotification {
    /** Short name of the prerequisite task (e.g. `xna-4.0`) */
    name: string;
    /** Current status of the prereq */
    status: PrereqStatus;
    /** Value between 0 and 1 (floating) */
    progress: number;
    /** ETA in seconds (floating) */
    eta: number;
    /** Network bandwidth used in bytes per second (floating) */
    bps: number;
}
/**
 * Current status of a prerequisite task
 *
 * Sent during @@LaunchParams, after @@PrereqsStartedNotification, repeatedly
 * until all prereq tasks are done.
 */
export declare const PrereqsTaskState: import("./support").NotificationCreator<PrereqsTaskStateNotification>;
/**
 * undocumented
 */
export declare enum PrereqStatus {
    Pending = "pending",
    Downloading = "downloading",
    Ready = "ready",
    Installing = "installing",
    Done = "done"
}
/**
 * Payload for PrereqsEnded
 */
export interface PrereqsEndedNotification {
}
/**
 * Sent during @@LaunchParams, when all prereqs have finished installing (successfully or not)
 *
 * After this is received, it's safe to close any UI element showing prereq task state.
 */
export declare const PrereqsEnded: import("./support").NotificationCreator<PrereqsEndedNotification>;
/**
 * Params for PrereqsFailed
 */
export interface PrereqsFailedParams {
    /** Short error */
    error: string;
    /** Longer error (to include in logs) */
    errorStack: string;
}
/**
 * Params for CleanDownloads.Search
 */
export interface CleanDownloadsSearchParams {
    /** A list of folders to scan for potential subfolders to clean up */
    roots: string[];
    /**
     * A list of subfolders to not consider when cleaning
     * (staging folders for in-progress downloads)
     */
    whitelist: string[];
}
/**
 * Result for CleanDownloads.Search
 */
export interface CleanDownloadsSearchResult {
    /** Entries we found that could use some cleaning (with path and size information) */
    entries: CleanDownloadsEntry[];
}
/**
 * Look for folders we can clean up in various download folders.
 * This finds anything that doesn't correspond to any current downloads
 * we know about.
 */
export declare const CleanDownloadsSearch: import("./support").RequestCreator<CleanDownloadsSearchParams, CleanDownloadsSearchResult>;
/**
 * undocumented
 */
export interface CleanDownloadsEntry {
    /** The complete path of the file or folder we intend to remove */
    path: string;
    /** The size of the folder or file, in bytes */
    size: number;
}
/**
 * Params for CleanDownloads.Apply
 */
export interface CleanDownloadsApplyParams {
    /** undocumented */
    entries: CleanDownloadsEntry[];
}
/**
 * Result for CleanDownloads.Apply
 */
export interface CleanDownloadsApplyResult {
}
/**
 * Remove the specified entries from disk, freeing up disk space.
 */
export declare const CleanDownloadsApply: import("./support").RequestCreator<CleanDownloadsApplyParams, CleanDownloadsApplyResult>;
/**
 * Params for System.StatFS
 */
export interface SystemStatFSParams {
    /** undocumented */
    path: string;
}
/**
 * Params for Test.DoubleTwice
 */
export interface TestDoubleTwiceParams {
    /** The number to quadruple */
    number: number;
}
/**
 * Result for Test.DoubleTwice
 */
export interface TestDoubleTwiceResult {
    /** The input, quadrupled */
    number: number;
}
/**
 * Test request: asks butler to double a number twice.
 * First by calling @@TestDoubleParams, then by
 * returning the result of that call doubled.
 *
 * Use that to try out your JSON-RPC 2.0 over TCP implementation.
 */
export declare const TestDoubleTwice: import("./support").RequestCreator<TestDoubleTwiceParams, TestDoubleTwiceResult>;
/**
 * Params for Test.Double
 */
export interface TestDoubleParams {
    /** The number to double */
    number: number;
}
