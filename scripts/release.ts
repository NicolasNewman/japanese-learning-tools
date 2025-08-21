import { execSync } from 'child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { exit } from 'process';

type CommitType = 'feat' | 'fix' | 'docs' | 'style' | 'refactor' | 'perf' | 'test' | 'build' | 'chore' | 'revert';
type VersionType = 'major' | 'minor' | 'patch';
type ProjectType = 'subs2clipboard-native-messenger' | 'subs2clipboard' | 'jp-learning-tools' | 'gd-sudachi' | 'gd-tools' | 'kanji-bank-types';

const commitTypeToVersion: Record<CommitType, VersionType> = {
    feat: 'minor',
    fix: 'patch',
    docs: 'patch',
    style: 'patch',
    refactor: 'patch',
    perf: 'patch',
    test: 'patch',
    build: 'patch',
    chore: 'patch',
    revert: 'patch'
}

const projectTypeToSourceDir: Record<ProjectType, string | Record<'js' | 'toml', string> | string[]> = {
    'jp-learning-tools': {
        'js': 'apps/jp-learning-tools/package.json',
        'toml': 'apps/jp-learning-tools/src-tauri/Cargo.toml'
    },
    'subs2clipboard-native-messenger': 'apps/subs2clipboard-native-messenger/Cargo.toml',
    'subs2clipboard': ['apps/subs2clipboard/package.json', 'apps/subs2clipboard/manifest.json'],
    'gd-sudachi': 'apps/gd-sudachi/pyproject.toml',
    'gd-tools': 'apps/gd-tools/Cargo.toml',
    'kanji-bank-types': 'packages/kanji-bank-types/package.json'
}

const bumpVersion = (version: [number, number, number], bumpType: VersionType): [number, number, number] => {
    switch (bumpType) {
        case 'major':
            version[0]++;
            version[1] = 0;
            version[2] = 0;
            break;
        case 'minor':
            version[1]++;
            version[2] = 0;
            break;
        case 'patch':
            version[2]++;
            break;
    }
    return version;
}

const resolveJavaScriptVersion = (projectType: ProjectType, bumpType: VersionType): [string, string] => {
    const src = projectTypeToSourceDir[projectType]
    const resolvedSrcArray = Array.isArray(src) ? src : [typeof src === 'string' ? src : src.js];
    const versions: { from: string | null, to: string | null } = { from: null, to: null };
    resolvedSrcArray.forEach(resolvedSrc => {
        const metadata = JSON.parse(readFileSync(resolvedSrc, 'utf8')) as Record<string, any>;
        versions.from ??= metadata.version;
        if (!versions.to) {
            const version = metadata.version.split('.').map(Number) as [number, number, number];
            bumpVersion(version, bumpType);
            versions.to = version.join('.');
        }
        metadata.version = versions.to;
        writeFileSync(resolvedSrc, JSON.stringify(metadata, null, 2));

    });
    // const resolvedSrc = typeof src === 'string' ? src : src.js;
    return [versions.from ?? '', versions.to ?? ''];
}

const resolveTomlVersion = (projectType: ProjectType, bumpType: VersionType): [string, string] => {
    const src = projectTypeToSourceDir[projectType]
    const resolvedSrc = typeof src === 'string' ? src : (src as Record<'js' | 'toml', string>).toml;
    const metadata = readFileSync(resolvedSrc, 'utf8');
    const versionFromTo: [string, string] = ["", ""]
    const lines = metadata.split('\n').map(line => {
        const match = line.match(/^version = \"([^ ]*)\"$/)
        if (match) {
            versionFromTo[0] = match[1];
            const version = match[1].split('.').map(Number) as [number, number, number];
            bumpVersion(version, bumpType);
            versionFromTo[1] = version.join('.');
            return `version = "${version.join('.')}"`;
        }
        return line;
    }).join('\n');
    writeFileSync(resolvedSrc, lines);
    if (resolvedSrc.split('/').pop() === 'Cargo.toml') {
        execSync(`cargo update -p ${projectType === 'jp-learning-tools' ? 'jp-learning-tools-backend' : projectType}`);
    }
    return versionFromTo;
}

const projectTypeToVersionResolver: Record<ProjectType, typeof resolveTomlVersion> = {
    'jp-learning-tools': (projectType: ProjectType, bumpType: VersionType) => {
        const jsChange = resolveJavaScriptVersion(projectType, bumpType);
        resolveTomlVersion(projectType, bumpType);
        return jsChange;
    },
    'subs2clipboard-native-messenger': resolveTomlVersion,
    'subs2clipboard': resolveJavaScriptVersion,
    'gd-sudachi': resolveTomlVersion,
    'gd-tools': resolveTomlVersion,
    'kanji-bank-types': resolveJavaScriptVersion
}

try {
    const output = execSync('git log origin/main..HEAD --oneline --no-decorate', {
        encoding: 'utf8',
        cwd: process.cwd()
    });
    console.log('Git log output:', output);
    const commits = output
        .trim()
        .split('\n')
        // Filter out empty lines and map to commit objects
        .filter(line => line.length > 0 && !line.substring(8).startsWith('Merge branch')).map(line => {
            const commit = line.substring(8);
            const parts = commit.match(/^([A-z]*)(\(([\w\d-]*)\))?: ?(.*)$/);
            if (!parts) {
                console.error(`Invalid commit format: ${commit}, exiting...`);
                exit(1);
            }
            return {
                type: parts[1],
                scope: parts[3],
                subject: parts[4]
            };
            // Re-organize commits by scope
        }).reduce((prev, commit) => {
            const scope = commit.scope || 'repo'; // Default to 'repo' if no scope is provided
            prev[scope] ??= [];
            prev[scope].push({
                type: commit.type,
                subject: commit.subject
            });

            return prev;
        }, {} as Record<ProjectType, { type: string, subject: string }[]>);

    console.log('Commits:', commits);

    // Generate patch notes and version changes
    const patchNotes: string[] = [];
    const allVersions: Partial<Record<VersionType, VersionType>> = {};
    Object.entries(commits).forEach(([scope, commitList]) => {
        const patchNote = [`## ${scope}\n`];
        const versions: Partial<Record<VersionType, VersionType>> = {}
        commitList.forEach(commit => {
            const type = commit.type as CommitType;
            versions[commitTypeToVersion[type]] = commitTypeToVersion[type];
            if (scope !== 'repo') {
                allVersions[commitTypeToVersion[type]] = commitTypeToVersion[type];
            }
            patchNote.push(`- ${commit.type}: ${commit.subject}`);
        });
        patchNotes.push(patchNote.join('\n'));
        const resolver = projectTypeToVersionResolver[scope as ProjectType];
        if (scope !== 'repo' && !resolver) {
            console.error(`No version resolver found for project type: ${scope}`);
            exit(1);
        }

        // Update application manifest version
        if (scope !== 'repo') {
            const change = resolver(scope as ProjectType, versions['major'] || versions['minor'] || 'patch' as VersionType);
            console.log(`${scope}: ${change[0]} -> ${change[1]}`);
        }
    });
    console.log('\n\nPatch Notes:\n')
    console.log(patchNotes.join('\n\n'));

    const jpLearningToolsVersion = JSON.parse(readFileSync('./package.json', 'utf8'));
    const version = jpLearningToolsVersion.version.split('.').map(Number) as [number, number, number];
    const newVersion = bumpVersion(version, allVersions['major'] || allVersions['minor'] || 'patch' as VersionType).join('.');
    jpLearningToolsVersion.version = newVersion;
    writeFileSync('./package.json', JSON.stringify(jpLearningToolsVersion, null, 2));

    const desktopAppReleaseVersion = JSON.parse(readFileSync('apps/jp-learning-tools/src-tauri/tauri.conf.json', 'utf8'));
    desktopAppReleaseVersion.version = newVersion;
    writeFileSync('apps/jp-learning-tools/src-tauri/tauri.conf.json', JSON.stringify(desktopAppReleaseVersion, null, 2));

    if (!existsSync('.changelog')) {
        mkdirSync('.changelog');
    }
    writeFileSync('.changelog/CHANGELOG.md', `# v${newVersion}\n\n${patchNotes.join('\n\n')}\n`);

    console.log(newVersion);
} catch (error) {
    console.error('Error executing git command:', error.message);
}