import {
    resourceDir,
    appDataDir,
    homeDir,
    sep
} from "@tauri-apps/api/path";
import {
    type
} from "@tauri-apps/plugin-os";
import {
    exists,
    mkdir,
    readTextFile,
    writeFile
} from "@tauri-apps/plugin-fs";
import { externalBinaryDir } from "./commands";

interface Manifest {
    name: string;
    description: string;
    path: string;
    type: string;
    allowed_origins: string[];
}

type BrowserData<T> = {
    chrome: T;
    firefox: T;
}

const getManifestPath = async (): Promise<BrowserData<string> | null> => {
    const os = type();
    const manifestName = "subs2clipboard_native_messenger";
    const home = await homeDir();
    const appData = await appDataDir();
    switch (os) {
        case "windows":
            return {
                chrome: `${appData}/Google/Chrome/User Data/NativeMessagingHosts/${manifestName}.json`,
                firefox: `${appData}\\manifest-firefox.json`
            };
        case 'linux':
            return {
                chrome: `${home}/.config/google-chrome/NativeMessagingHosts/${manifestName}.json`,
                firefox: `${home}/.mozilla/native-messaging-hosts/${manifestName}.json`
            }
        case 'macos':
            return {
                chrome: `${home}/Library/Application Support/Google/Chrome/NativeMessagingHosts/${manifestName}.json`,
                firefox: `${home}/Library/Application Support/Mozilla/NativeMessagingHosts/${manifestName}.json`
            }
        default:
            return null;
    }
}

const isManifestInstalled = async (browser: keyof BrowserData<any>) => {
    const manifestPath = await getManifestPath();
    if (!manifestPath) {
        return false;
    }
    if (!await exists(manifestPath[browser])) {
        return false;
    }
    return true;
}

enum InstallManifestStatusCode {
    SUCCESS = "SUCCESS",
    ALREADY_INSTALLED = "ALREADY_INSTALLED",
    NOT_SUPPORTED_OS = "NOT_SUPPORTED_OS",
    MANIFEST_FILE_NOT_FOUND = "MANIFEST_FILE_NOT_FOUND",
}

const installManifest = async (): Promise<InstallManifestStatusCode> => {
    if (!await isManifestInstalled("firefox")) {
        const resourcePath = await resourceDir();
        const manifestPath = await getManifestPath();
        if (!manifestPath) {
            console.error("Unsupported OS for manifest installation");
            throw new Error(InstallManifestStatusCode.NOT_SUPPORTED_OS, { cause: "Unsupported OS for manifest installation" });
        }
        console.log(`Manifest path for Firefox: ${manifestPath["firefox"]}`);
        if (await exists(manifestPath["firefox"])) {
            return InstallManifestStatusCode.ALREADY_INSTALLED;
        }

        const manifestDir = manifestPath["firefox"].substring(0, manifestPath["firefox"].lastIndexOf(sep()));
        console.log(`Checking if directory exists: ${manifestDir}`);
        if (type() !== 'windows' && !await exists(manifestDir)) {
            await mkdir(manifestDir, { recursive: true });
        }

        const manifestFile = `${resourcePath}${sep()}resources${sep()}manifest${sep()}manifest-firefox.json`;
        console.log(`Checking if file exists: ${manifestFile}`);
        if (!await exists(manifestFile)) {
            console.error("Manifest file not found in resources");
            throw new Error(InstallManifestStatusCode.MANIFEST_FILE_NOT_FOUND, { cause: `Manifest file not found in resources: ${manifestFile}` });
        }

        const manifestJson = JSON.parse(await readTextFile(manifestFile)) as Manifest;
        manifestJson.path = `${await externalBinaryDir()}${sep()}subs2clipboard-native-messenger${type() === 'windows' ? '.exe' : ''}`;

        const encoder = new TextEncoder();
        await writeFile(manifestPath["firefox"], encoder.encode(JSON.stringify(manifestJson, null, 4)));
    }

    return InstallManifestStatusCode.SUCCESS;
};

export { installManifest }