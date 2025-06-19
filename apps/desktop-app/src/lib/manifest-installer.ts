import {
    resourceDir,
    appDataDir,
    homeDir
} from "@tauri-apps/api/path";
import {
    type
} from "@tauri-apps/plugin-os";
import {
    exists,
    copyFile,
    mkdir
} from "@tauri-apps/plugin-fs";

const getManifestPath = async () => {
    const os = type();
    const manifestName = "subs2srs";
    switch (os) {
        case "windows":
            return ``;
        case 'linux':
            const home = await homeDir();
            return `${home}/.mozilla/native-messaging-hosts/${manifestName}.json`;
        case 'macos':
            return `/Library/Application Support/Mozilla/NativeMessagingHosts/${manifestName}.json`;
        default:
            return null;
    }
}

const isManifestInstalled = async () => {
    const manifestPath = await getManifestPath();
    if (!manifestPath) {
        return false;
    }
    if (!await exists(manifestPath)) {
        return false;
    }
    return true;
}

enum InstallManifestStatusCode {
    SUCCESS = 0,
    ALREADY_INSTALLED = 1,
    NOT_SUPPORTED_OS = 2,
    MANIFEST_FILE_NOT_FOUND = 3,
}

const installManifest = async (): Promise<InstallManifestStatusCode> => {
    if (!await isManifestInstalled()) {
        const resourcePath = await resourceDir();
        const manifestPath = await getManifestPath();
        if (!manifestPath) {
            console.error("Unsupported OS for manifest installation");
            throw new Error("Unsupported OS for manifest installation");
        }
        if (await exists(manifestPath)) {
            return InstallManifestStatusCode.ALREADY_INSTALLED;
        }
        
        // Create the directory if it doesn't exist
        const home = await homeDir();
        const manifestDir = `${home}/.mozilla/native-messaging-hosts`;
        if (!await exists(manifestDir)) {
            await mkdir(manifestDir, { recursive: true });
        }
        
        const manifestFile = `${resourcePath}/resources/manifest/manifest-firefox.json`;
        if (!await exists(manifestFile)) {
            console.error("Manifest file not found in resources");
            throw new Error(`Manifest file not found in resources: ${manifestFile}`);
        }
        await copyFile(manifestFile, manifestPath);
    }

    return InstallManifestStatusCode.SUCCESS;
};

export { installManifest }