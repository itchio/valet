"use strict";
// These bindings were generated by generous
// See <https://docs.itch.ovh/butlerd/master/> for a human-friendly documentation
exports.__esModule = true;
var support_1 = require("./support");
/**
 * undocumented
 */
var LaunchStrategy;
(function (LaunchStrategy) {
    LaunchStrategy["Unknown"] = "";
    LaunchStrategy["Native"] = "native";
    LaunchStrategy["HTML"] = "html";
    LaunchStrategy["URL"] = "url";
    LaunchStrategy["Shell"] = "shell";
})(LaunchStrategy = exports.LaunchStrategy || (exports.LaunchStrategy = {}));
/**
 * When using TCP transport, must be the first message sent
 */
exports.MetaAuthenticate = support_1.createRequest("Meta.Authenticate");
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
exports.MetaFlow = support_1.createRequest("Meta.Flow");
/**
 * When called, gracefully shutdown the butler daemon.
 */
exports.MetaShutdown = support_1.createRequest("Meta.Shutdown");
/**
 * Retrieves the version of the butler instance the client
 * is connected to.
 *
 * This endpoint is meant to gather information when reporting
 * issues, rather than feature sniffing. Conforming clients should
 * automatically download new versions of butler, see the **Updating** section.
 */
exports.VersionGet = support_1.createRequest("Version.Get");
/**
 * undocumented
 */
exports.NetworkSetSimulateOffline = support_1.createRequest("Network.SetSimulateOffline");
/**
 * undocumented
 */
exports.NetworkSetBandwidthThrottle = support_1.createRequest("Network.SetBandwidthThrottle");
/**
 * Lists remembered profiles
 */
exports.ProfileList = support_1.createRequest("Profile.List");
/**
 * Add a new profile by password login
 */
exports.ProfileLoginWithPassword = support_1.createRequest("Profile.LoginWithPassword");
/**
 * Add a new profile by API key login. This can be used
 * for integration tests, for example. Note that no cookies
 * are returned for this kind of login.
 */
exports.ProfileLoginWithAPIKey = support_1.createRequest("Profile.LoginWithAPIKey");
/**
 * Ask the user to solve a captcha challenge
 * Sent during @@ProfileLoginWithPasswordParams if certain
 * conditions are met.
 */
exports.ProfileRequestCaptcha = support_1.createRequest("Profile.RequestCaptcha");
/**
 * Ask the user to provide a TOTP token.
 * Sent during @@ProfileLoginWithPasswordParams if the user has
 * two-factor authentication enabled.
 */
exports.ProfileRequestTOTP = support_1.createRequest("Profile.RequestTOTP");
/**
 * Use saved login credentials to validate a profile.
 */
exports.ProfileUseSavedLogin = support_1.createRequest("Profile.UseSavedLogin");
/**
 * Forgets a remembered profile - it won't appear in the
 * @@ProfileListParams results anymore.
 */
exports.ProfileForget = support_1.createRequest("Profile.Forget");
/**
 * Stores some data associated to a profile, by key.
 */
exports.ProfileDataPut = support_1.createRequest("Profile.Data.Put");
/**
 * Retrieves some data associated to a profile, by key.
 */
exports.ProfileDataGet = support_1.createRequest("Profile.Data.Get");
/**
 * Searches for games.
 */
exports.SearchGames = support_1.createRequest("Search.Games");
/**
 * Searches for users.
 */
exports.SearchUsers = support_1.createRequest("Search.Users");
/**
 * Fetches information for an itch.io game.
 */
exports.FetchGame = support_1.createRequest("Fetch.Game");
/**
 * undocumented
 */
var GameRecordsSource;
(function (GameRecordsSource) {
    // Games for which the profile has a download key
    GameRecordsSource["Owned"] = "owned";
    // Games for which a cave exists (regardless of the profile)
    GameRecordsSource["Installed"] = "installed";
    // Games authored by profile, or for whom profile is an admin of
    GameRecordsSource["Profile"] = "profile";
    // Games from a collection
    GameRecordsSource["Collection"] = "collection";
})(GameRecordsSource = exports.GameRecordsSource || (exports.GameRecordsSource = {}));
/**
 * Fetches game records - owned, installed, in collection,
 * with search, etc. Includes download key info, cave info, etc.
 */
exports.FetchGameRecords = support_1.createRequest("Fetch.GameRecords");
/**
 * Fetches a download key
 */
exports.FetchDownloadKey = support_1.createRequest("Fetch.DownloadKey");
/**
 * Fetches multiple download keys
 */
exports.FetchDownloadKeys = support_1.createRequest("Fetch.DownloadKeys");
/**
 * Fetches uploads for an itch.io game
 */
exports.FetchGameUploads = support_1.createRequest("Fetch.GameUploads");
/**
 * Fetches information for an itch.io user.
 */
exports.FetchUser = support_1.createRequest("Fetch.User");
/**
 * Fetches the best current *locally cached* sale for a given
 * game.
 */
exports.FetchSale = support_1.createRequest("Fetch.Sale");
/**
 * Fetch a collection's title, gamesCount, etc.
 * but not its games.
 */
exports.FetchCollection = support_1.createRequest("Fetch.Collection");
/**
 * Fetches information about a collection and the games it
 * contains.
 */
exports.FetchCollectionGames = support_1.createRequest("Fetch.Collection.Games");
/**
 * Lists collections for a profile. Does not contain
 * games.
 */
exports.FetchProfileCollections = support_1.createRequest("Fetch.ProfileCollections");
/**
 * undocumented
 */
exports.FetchProfileGames = support_1.createRequest("Fetch.ProfileGames");
/**
 * undocumented
 */
exports.FetchProfileOwnedKeys = support_1.createRequest("Fetch.ProfileOwnedKeys");
/**
 * undocumented
 */
exports.FetchCommons = support_1.createRequest("Fetch.Commons");
/**
 * Retrieve info for all caves.
 */
exports.FetchCaves = support_1.createRequest("Fetch.Caves");
/**
 * Retrieve info on a cave by ID.
 */
exports.FetchCave = support_1.createRequest("Fetch.Cave");
/**
 * Mark all local data as stale.
 */
exports.FetchExpireAll = support_1.createRequest("Fetch.ExpireAll");
/**
 * Finds uploads compatible with the current runtime, for a given game.
 */
exports.GameFindUploads = support_1.createRequest("Game.FindUploads");
/**
 * Queues an install operation to be later performed
 * via @@InstallPerformParams.
 */
exports.InstallQueue = support_1.createRequest("Install.Queue");
/**
 * For modal-first install
 */
exports.InstallPlan = support_1.createRequest("Install.Plan");
/**
 * undocumented
 */
exports.CavesSetPinned = support_1.createRequest("Caves.SetPinned");
/**
 * Create a shortcut for an existing cave .
 */
exports.InstallCreateShortcut = support_1.createRequest("Install.CreateShortcut");
/**
 * Perform an install that was previously queued via
 * @@InstallQueueParams.
 *
 * Can be cancelled by passing the same `ID` to @@InstallCancelParams.
 */
exports.InstallPerform = support_1.createRequest("Install.Perform");
/**
 * Attempt to gracefully cancel an ongoing operation.
 */
exports.InstallCancel = support_1.createRequest("Install.Cancel");
/**
 * UninstallParams contains all the parameters needed to perform
 * an uninstallation for a game via @@OperationStartParams.
 */
exports.UninstallPerform = support_1.createRequest("Uninstall.Perform");
/**
 * Prepare to queue a version switch. The client will
 * receive an @@InstallVersionSwitchPickParams.
 */
exports.InstallVersionSwitchQueue = support_1.createRequest("Install.VersionSwitch.Queue");
/**
 * Let the user pick which version to switch to.
 */
exports.InstallVersionSwitchPick = support_1.createRequest("InstallVersionSwitchPick");
/**
 * Asks the user to pick between multiple available uploads
 */
exports.PickUpload = support_1.createRequest("PickUpload");
/**
 * undocumented
 */
exports.InstallLocationsList = support_1.createRequest("Install.Locations.List");
/**
 * undocumented
 */
exports.InstallLocationsAdd = support_1.createRequest("Install.Locations.Add");
/**
 * undocumented
 */
exports.InstallLocationsRemove = support_1.createRequest("Install.Locations.Remove");
/**
 * undocumented
 */
exports.InstallLocationsGetByID = support_1.createRequest("Install.Locations.GetByID");
/**
 * Sent at the end of @@InstallLocationsScanParams
 */
exports.InstallLocationsScanConfirmImport = support_1.createRequest("Install.Locations.Scan.ConfirmImport");
/**
 * undocumented
 */
exports.InstallLocationsScan = support_1.createRequest("Install.Locations.Scan");
/**
 * Queue a download that will be performed later by
 * @@DownloadsDriveParams.
 */
exports.DownloadsQueue = support_1.createRequest("Downloads.Queue");
/**
 * Put a download on top of the queue.
 */
exports.DownloadsPrioritize = support_1.createRequest("Downloads.Prioritize");
/**
 * List all known downloads.
 */
exports.DownloadsList = support_1.createRequest("Downloads.List");
/**
 * Removes all finished downloads from the queue.
 */
exports.DownloadsClearFinished = support_1.createRequest("Downloads.ClearFinished");
/**
 * Drive downloads, which is: perform them one at a time,
 * until they're all finished.
 */
exports.DownloadsDrive = support_1.createRequest("Downloads.Drive");
/**
 * Stop driving downloads gracefully.
 */
exports.DownloadsDriveCancel = support_1.createRequest("Downloads.Drive.Cancel");
/**
 * undocumented
 */
exports.DownloadsDriveProgress = support_1.createNotification("Downloads.Drive.Progress");
/**
 * undocumented
 */
exports.DownloadsDriveStarted = support_1.createNotification("Downloads.Drive.Started");
/**
 * undocumented
 */
exports.DownloadsDriveErrored = support_1.createNotification("Downloads.Drive.Errored");
/**
 * undocumented
 */
exports.DownloadsDriveFinished = support_1.createNotification("Downloads.Drive.Finished");
/**
 * undocumented
 */
exports.DownloadsDriveDiscarded = support_1.createNotification("Downloads.Drive.Discarded");
/**
 * Sent during @@DownloadsDriveParams to inform on network
 * status changes.
 */
exports.DownloadsDriveNetworkStatus = support_1.createNotification("Downloads.Drive.NetworkStatus");
/**
 * undocumented
 */
var NetworkStatus;
(function (NetworkStatus) {
    NetworkStatus["Online"] = "online";
    NetworkStatus["Offline"] = "offline";
})(NetworkStatus = exports.NetworkStatus || (exports.NetworkStatus = {}));
/**
 * undocumented
 */
var DownloadReason;
(function (DownloadReason) {
    DownloadReason["Install"] = "install";
    DownloadReason["Reinstall"] = "reinstall";
    DownloadReason["Update"] = "update";
    DownloadReason["VersionSwitch"] = "version-switch";
})(DownloadReason = exports.DownloadReason || (exports.DownloadReason = {}));
/**
 * Retries a download that has errored
 */
exports.DownloadsRetry = support_1.createRequest("Downloads.Retry");
/**
 * Attempts to discard a download
 */
exports.DownloadsDiscard = support_1.createRequest("Downloads.Discard");
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
exports.CheckUpdate = support_1.createRequest("CheckUpdate");
/**
 * Snoozing a cave means we ignore all new uploads (that would
 * be potential updates) between the cave's last install operation
 * and now.
 *
 * This can be undone by calling @@CheckUpdateParams with this specific
 * cave identifier.
 */
exports.SnoozeCave = support_1.createRequest("SnoozeCave");
/**
 * Attempt to launch an installed game.
 */
exports.Launch = support_1.createRequest("Launch");
/**
 * Sent during @@LaunchParams if the game/application comes with a service license
 * agreement.
 */
exports.AcceptLicense = support_1.createRequest("AcceptLicense");
/**
 * Sent during @@LaunchParams, ask the user to pick a manifest action to launch.
 *
 * See [itch app manifests](https://itch.io/docs/itch/integrating/manifest.html).
 */
exports.PickManifestAction = support_1.createRequest("PickManifestAction");
/**
 * Ask the client to perform a shell launch, ie. open an item
 * with the operating system's default handler (File explorer).
 *
 * Sent during @@LaunchParams.
 */
exports.ShellLaunch = support_1.createRequest("ShellLaunch");
/**
 * Ask the client to perform an HTML launch, ie. open an HTML5
 * game, ideally in an embedded browser.
 *
 * Sent during @@LaunchParams.
 */
exports.HTMLLaunch = support_1.createRequest("HTMLLaunch");
/**
 * Ask the client to perform an URL launch, ie. open an address
 * with the system browser or appropriate.
 *
 * Sent during @@LaunchParams.
 */
exports.URLLaunch = support_1.createRequest("URLLaunch");
/**
 * Ask the user to allow sandbox setup. Will be followed by
 * a UAC prompt (on Windows) or a pkexec dialog (on Linux) if
 * the user allows.
 *
 * Sent during @@LaunchParams.
 */
exports.AllowSandboxSetup = support_1.createRequest("AllowSandboxSetup");
/**
 * Sent during @@LaunchParams, when one or more prerequisites have failed to install.
 * The user may choose to proceed with the launch anyway.
 */
exports.PrereqsFailed = support_1.createRequest("PrereqsFailed");
/**
 * Get information on a filesystem.
 */
exports.SystemStatFS = support_1.createRequest("System.StatFS");
/**
 * Sent any time butler needs to send a log message. The client should
 * relay them in their own stdout / stderr, and collect them so they
 * can be part of an issue report if something goes wrong.
 */
exports.Log = support_1.createNotification("Log");
/**
 * undocumented
 */
var LogLevel;
(function (LogLevel) {
    // Hidden from logs by default, noisy
    LogLevel["Debug"] = "debug";
    // Just thinking out loud
    LogLevel["Info"] = "info";
    // We're continuing, but we're not thrilled about it
    LogLevel["Warning"] = "warning";
    // We're eventually going to fail loudly
    LogLevel["Error"] = "error";
})(LogLevel = exports.LogLevel || (exports.LogLevel = {}));
/**
 * Test request: return a number, doubled. Implement that to
 * use @@TestDoubleTwiceParams in your testing.
 */
exports.TestDouble = support_1.createRequest("Test.Double");
/**
 * butlerd JSON-RPC 2.0 error codes
 */
var Code;
(function (Code) {
    // An operation was cancelled gracefully
    Code[Code["OperationCancelled"] = 499] = "OperationCancelled";
    // An operation was aborted by the user
    Code[Code["OperationAborted"] = 410] = "OperationAborted";
    // We tried to launch something, but the install folder just wasn't there
    Code[Code["InstallFolderDisappeared"] = 404] = "InstallFolderDisappeared";
    // We tried to install something, but could not find compatible uploads
    Code[Code["NoCompatibleUploads"] = 2001] = "NoCompatibleUploads";
    // This title is hosted on an incompatible third-party website
    Code[Code["UnsupportedHost"] = 3001] = "UnsupportedHost";
    // Nothing that can be launched was found
    Code[Code["NoLaunchCandidates"] = 5000] = "NoLaunchCandidates";
    // Java Runtime Environment is required to launch this title.
    Code[Code["JavaRuntimeNeeded"] = 6000] = "JavaRuntimeNeeded";
    // There is no Internet connection
    Code[Code["NetworkDisconnected"] = 9000] = "NetworkDisconnected";
    // API error
    Code[Code["APIError"] = 12000] = "APIError";
    // The database is busy
    Code[Code["DatabaseBusy"] = 16000] = "DatabaseBusy";
    // An install location could not be removed because it has active downloads
    Code[Code["CantRemoveLocationBecauseOfActiveDownloads"] = 18000] = "CantRemoveLocationBecauseOfActiveDownloads";
})(Code = exports.Code || (exports.Code = {}));
/**
 * Flavor describes whether we're dealing with a native executables, a Java archive, a love2d bundle, etc.
 */
var Flavor;
(function (Flavor) {
    // FlavorNativeLinux denotes native linux executables
    Flavor["NativeLinux"] = "linux";
    // ExecNativeMacos denotes native macOS executables
    Flavor["NativeMacos"] = "macos";
    // FlavorPe denotes native windows executables
    Flavor["NativeWindows"] = "windows";
    // FlavorAppMacos denotes a macOS app bundle
    Flavor["AppMacos"] = "app-macos";
    // FlavorScript denotes scripts starting with a shebang (#!)
    Flavor["Script"] = "script";
    // FlavorScriptWindows denotes windows scripts (.bat or .cmd)
    Flavor["ScriptWindows"] = "windows-script";
    // FlavorJar denotes a .jar archive with a Main-Class
    Flavor["Jar"] = "jar";
    // FlavorHTML denotes an index html file
    Flavor["HTML"] = "html";
    // FlavorLove denotes a love package
    Flavor["Love"] = "love";
    // Microsoft installer packages
    Flavor["MSI"] = "msi";
})(Flavor = exports.Flavor || (exports.Flavor = {}));
/**
 * The architecture of an executable
 */
var Arch;
(function (Arch) {
    // 32-bit
    Arch["_386"] = "386";
    // 64-bit
    Arch["Amd64"] = "amd64";
})(Arch = exports.Arch || (exports.Arch = {}));
/**
 * Which particular type of windows-specific installer
 */
var WindowsInstallerType;
(function (WindowsInstallerType) {
    // Microsoft install packages (`.msi` files)
    WindowsInstallerType["Msi"] = "msi";
    // InnoSetup installers
    WindowsInstallerType["Inno"] = "inno";
    // NSIS installers
    WindowsInstallerType["Nullsoft"] = "nsis";
    // Self-extracting installers that 7-zip knows how to extract
    WindowsInstallerType["Archive"] = "archive";
})(WindowsInstallerType = exports.WindowsInstallerType || (exports.WindowsInstallerType = {}));
/**
 * Architectures describes a set of processor architectures (mostly 32-bit vs 64-bit)
 */
var Architectures;
(function (Architectures) {
    // ArchitecturesAll represents any processor architecture
    Architectures["All"] = "all";
    // Architectures386 represents 32-bit processor architectures
    Architectures["_386"] = "386";
    // ArchitecturesAmd64 represents 64-bit processor architectures
    Architectures["Amd64"] = "amd64";
})(Architectures = exports.Architectures || (exports.Architectures = {}));
/**
 * GameType is the type of an itch.io game page, mostly related to
 * how it should be presented on web (downloadable or embed)
 */
var GameType;
(function (GameType) {
    // GameTypeDefault is downloadable games
    GameType["Default"] = "default";
    // GameTypeFlash is for .swf (legacy)
    GameType["Flash"] = "flash";
    // GameTypeUnity is for .unity3d (legacy)
    GameType["Unity"] = "unity";
    // GameTypeJava is for .jar (legacy)
    GameType["Java"] = "java";
    // GameTypeHTML is for .html (thriving)
    GameType["HTML"] = "html";
})(GameType = exports.GameType || (exports.GameType = {}));
/**
 * GameClassification is the creator-picked classification for a page
 */
var GameClassification;
(function (GameClassification) {
    // GameClassificationGame is something you can play
    GameClassification["Game"] = "game";
    // GameClassificationTool includes all software pretty much
    GameClassification["Tool"] = "tool";
    // GameClassificationAssets includes assets: graphics, sounds, etc.
    GameClassification["Assets"] = "assets";
    // GameClassificationGameMod are game mods (no link to game, purely creator tagging)
    GameClassification["GameMod"] = "game_mod";
    // GameClassificationPhysicalGame is for a printable / board / card game
    GameClassification["PhysicalGame"] = "physical_game";
    // GameClassificationSoundtrack is a bunch of music files
    GameClassification["Soundtrack"] = "soundtrack";
    // GameClassificationOther is anything that creators think don't fit in any other category
    GameClassification["Other"] = "other";
    // GameClassificationComic is a comic book (pdf, jpg, specific comic formats, etc.)
    GameClassification["Comic"] = "comic";
    // GameClassificationBook is a book (pdf, jpg, specific e-book formats, etc.)
    GameClassification["Book"] = "book";
})(GameClassification = exports.GameClassification || (exports.GameClassification = {}));
/**
 * UploadStorage describes where an upload file is stored.
 */
var UploadStorage;
(function (UploadStorage) {
    // UploadStorageHosted is a classic upload (web) - no versioning
    UploadStorage["Hosted"] = "hosted";
    // UploadStorageBuild is a wharf upload (butler)
    UploadStorage["Build"] = "build";
    // UploadStorageExternal is an external upload - alllllllll bets are off.
    UploadStorage["External"] = "external";
})(UploadStorage = exports.UploadStorage || (exports.UploadStorage = {}));
/**
 * UploadType describes what's in an upload - an executable,
 * a web game, some music, etc.
 */
var UploadType;
(function (UploadType) {
    // UploadTypeDefault is for executables
    UploadType["Default"] = "default";
    // UploadTypeFlash is for .swf files
    UploadType["Flash"] = "flash";
    // UploadTypeUnity is for .unity3d files
    UploadType["Unity"] = "unity";
    // UploadTypeJava is for .jar files
    UploadType["Java"] = "java";
    // UploadTypeHTML is for .html files
    UploadType["HTML"] = "html";
    // UploadTypeSoundtrack is for archives with .mp3/.ogg/.flac/etc files
    UploadType["Soundtrack"] = "soundtrack";
    // UploadTypeBook is for books (epubs, pdfs, etc.)
    UploadType["Book"] = "book";
    // UploadTypeVideo is for videos
    UploadType["Video"] = "video";
    // UploadTypeDocumentation is for documentation (pdf, maybe uhh doxygen?)
    UploadType["Documentation"] = "documentation";
    // UploadTypeMod is a bunch of loose files with no clear instructions how to apply them to a game
    UploadType["Mod"] = "mod";
    // UploadTypeAudioAssets is a bunch of .ogg/.wav files
    UploadType["AudioAssets"] = "audio_assets";
    // UploadTypeGraphicalAssets is a bunch of .png/.svg/.gif files, maybe some .objs thrown in there
    UploadType["GraphicalAssets"] = "graphical_assets";
    // UploadTypeSourcecode is for source code. No further comments.
    UploadType["Sourcecode"] = "sourcecode";
    // UploadTypeOther is for literally anything that isn't an existing category,
    // or for stuff that isn't tagged properly.
    UploadType["Other"] = "other";
})(UploadType = exports.UploadType || (exports.UploadType = {}));
/**
 * BuildState describes the state of a build, relative to its initial upload, and
 * its processing.
 */
var BuildState;
(function (BuildState) {
    // BuildStateStarted is the state of a build from its creation until the initial upload is complete
    BuildState["Started"] = "started";
    // BuildStateProcessing is the state of a build from the initial upload's completion to its fully-processed state.
    // This state does not mean the build is actually being processed right now, it's just queued for processing.
    BuildState["Processing"] = "processing";
    // BuildStateCompleted means the build was successfully processed. Its patch hasn't necessarily been
    // rediff'd yet, but we have the holy (patch,signature,archive) trinity.
    BuildState["Completed"] = "completed";
    // BuildStateFailed means something went wrong with the build. A failing build will not update the channel
    // head and can be requeued by the itch.io team, although if a new build is pushed before they do,
    // that new build will "win".
    BuildState["Failed"] = "failed";
})(BuildState = exports.BuildState || (exports.BuildState = {}));
/**
 * BuildFileState describes the state of a specific file for a build
 */
var BuildFileState;
(function (BuildFileState) {
    // BuildFileStateCreated means the file entry exists on itch.io
    BuildFileState["Created"] = "created";
    // BuildFileStateUploading means the file is currently being uploaded to storage
    BuildFileState["Uploading"] = "uploading";
    // BuildFileStateUploaded means the file is ready
    BuildFileState["Uploaded"] = "uploaded";
    // BuildFileStateFailed means the file failed uploading
    BuildFileState["Failed"] = "failed";
})(BuildFileState = exports.BuildFileState || (exports.BuildFileState = {}));
/**
 * BuildFileType describes the type of a build file: patch, archive, signature, etc.
 */
var BuildFileType;
(function (BuildFileType) {
    // BuildFileTypePatch describes wharf patch files (.pwr)
    BuildFileType["Patch"] = "patch";
    // BuildFileTypeArchive describes canonical archive form (.zip)
    BuildFileType["Archive"] = "archive";
    // BuildFileTypeSignature describes wharf signature files (.pws)
    BuildFileType["Signature"] = "signature";
    // BuildFileTypeManifest is reserved
    BuildFileType["Manifest"] = "manifest";
    // BuildFileTypeUnpacked describes the single file that is in the build (if it was just a single file)
    BuildFileType["Unpacked"] = "unpacked";
})(BuildFileType = exports.BuildFileType || (exports.BuildFileType = {}));
/**
 * BuildFileSubType describes the subtype of a build file: mostly its compression
 * level. For example, rediff'd patches are "optimized", whereas initial patches are "default"
 */
var BuildFileSubType;
(function (BuildFileSubType) {
    // BuildFileSubTypeDefault describes default compression (rsync patches)
    BuildFileSubType["Default"] = "default";
    // BuildFileSubTypeGzip is reserved
    BuildFileSubType["Gzip"] = "gzip";
    // BuildFileSubTypeOptimized describes optimized compression (rediff'd / bsdiff patches)
    BuildFileSubType["Optimized"] = "optimized";
})(BuildFileSubType = exports.BuildFileSubType || (exports.BuildFileSubType = {}));
/**
 * undocumented
 */
var InstallEventType;
(function (InstallEventType) {
    // Started for the first time or resumed after a pause
    // or exit or whatever
    InstallEventType["InstallEventResume"] = "resume";
    // Stopped explicitly (pausing downloads), can't rely
    // on this being present because BRÜTAL PÖWER LÖSS will
    // not announce itself 🔥
    InstallEventType["InstallEventStop"] = "stop";
    // Regular install from archive or naked file
    InstallEventType["InstallEventInstall"] = "install";
    // Reverting to previous version or re-installing
    // wharf-powered upload
    InstallEventType["InstallEventHeal"] = "heal";
    // Applying one or more wharf patches
    InstallEventType["InstallEventUpgrade"] = "upgrade";
    // Applying a single wharf patch
    InstallEventType["InstallEventPatching"] = "patching";
    // Cleaning up ghost files
    InstallEventType["InstallEventGhostBusting"] = "ghostBusting";
    // Any kind of step failing
    InstallEventType["InstallEventProblem"] = "problem";
    // Any operation we do as a result of another one failing,
    // but in a case where we're still expecting a favorable
    // outcome eventually.
    InstallEventType["InstallEventFallback"] = "fallback";
})(InstallEventType = exports.InstallEventType || (exports.InstallEventType = {}));
/**
 * undocumented
 */
var Platform;
(function (Platform) {
    Platform["OSX"] = "osx";
    Platform["Windows"] = "windows";
    Platform["Linux"] = "linux";
    Platform["Unknown"] = "unknown";
})(Platform = exports.Platform || (exports.Platform = {}));
/**
 * The first notification sent when @@MetaFlowParams is called.
 */
exports.MetaFlowEstablished = support_1.createNotification("MetaFlowEstablished");
/**
 * Sent periodically during @@InstallPerformParams to inform on the current state of an install
 */
exports.Progress = support_1.createNotification("Progress");
/**
 * undocumented
 */
var TaskReason;
(function (TaskReason) {
    // Task was started for an install operation
    TaskReason["Install"] = "install";
    // Task was started for an uninstall operation
    TaskReason["Uninstall"] = "uninstall";
})(TaskReason = exports.TaskReason || (exports.TaskReason = {}));
/**
 * undocumented
 */
var TaskType;
(function (TaskType) {
    // We're fetching files from a remote server
    TaskType["Download"] = "download";
    // We're running an installer
    TaskType["Install"] = "install";
    // We're running an uninstaller
    TaskType["Uninstall"] = "uninstall";
    // We're applying some patches
    TaskType["Update"] = "update";
    // We're healing from a signature and heal source
    TaskType["Heal"] = "heal";
})(TaskType = exports.TaskType || (exports.TaskType = {}));
/**
 * Each operation is made up of one or more tasks. This notification
 * is sent during @@OperationStartParams whenever a specific task starts.
 */
exports.TaskStarted = support_1.createNotification("TaskStarted");
/**
 * Sent during @@OperationStartParams whenever a task succeeds for an operation.
 */
exports.TaskSucceeded = support_1.createNotification("TaskSucceeded");
/**
 * Sent during @@InstallLocationsScanParams whenever
 * a game is found.
 */
exports.InstallLocationsScanYield = support_1.createNotification("Install.Locations.Scan.Yield");
/**
 * Sent during @@CheckUpdateParams, every time butler
 * finds an update for a game. Can be safely ignored if displaying
 * updates as they are found is not a requirement for the client.
 */
exports.GameUpdateAvailable = support_1.createNotification("GameUpdateAvailable");
/**
 * Sent during @@LaunchParams, when the game is configured, prerequisites are installed
 * sandbox is set up (if enabled), and the game is actually running.
 */
exports.LaunchRunning = support_1.createNotification("LaunchRunning");
/**
 * Sent during @@LaunchParams, when the game has actually exited.
 */
exports.LaunchExited = support_1.createNotification("LaunchExited");
/**
 * Sent during @@LaunchParams, when some prerequisites are about to be installed.
 *
 * This is a good time to start showing a UI element with the state of prereq
 * tasks.
 *
 * Updates are regularly provided via @@PrereqsTaskStateNotification.
 */
exports.PrereqsStarted = support_1.createNotification("PrereqsStarted");
/**
 * Current status of a prerequisite task
 *
 * Sent during @@LaunchParams, after @@PrereqsStartedNotification, repeatedly
 * until all prereq tasks are done.
 */
exports.PrereqsTaskState = support_1.createNotification("PrereqsTaskState");
/**
 * undocumented
 */
var PrereqStatus;
(function (PrereqStatus) {
    // Prerequisite has not started downloading yet
    PrereqStatus["Pending"] = "pending";
    // Prerequisite is currently being downloaded
    PrereqStatus["Downloading"] = "downloading";
    // Prerequisite has been downloaded and is pending installation
    PrereqStatus["Ready"] = "ready";
    // Prerequisite is currently installing
    PrereqStatus["Installing"] = "installing";
    // Prerequisite was installed (successfully or not)
    PrereqStatus["Done"] = "done";
})(PrereqStatus = exports.PrereqStatus || (exports.PrereqStatus = {}));
/**
 * Sent during @@LaunchParams, when all prereqs have finished installing (successfully or not)
 *
 * After this is received, it's safe to close any UI element showing prereq task state.
 */
exports.PrereqsEnded = support_1.createNotification("PrereqsEnded");
/**
 * Look for folders we can clean up in various download folders.
 * This finds anything that doesn't correspond to any current downloads
 * we know about.
 */
exports.CleanDownloadsSearch = support_1.createRequest("CleanDownloads.Search");
/**
 * Remove the specified entries from disk, freeing up disk space.
 */
exports.CleanDownloadsApply = support_1.createRequest("CleanDownloads.Apply");
/**
 * Test request: asks butler to double a number twice.
 * First by calling @@TestDoubleParams, then by
 * returning the result of that call doubled.
 *
 * Use that to try out your JSON-RPC 2.0 over TCP implementation.
 */
exports.TestDoubleTwice = support_1.createRequest("Test.DoubleTwice");
