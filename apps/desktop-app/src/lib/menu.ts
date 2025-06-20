import { Menu, MenuItem, Submenu } from '@tauri-apps/api/menu';
import { goto } from '$app/navigation';


export default async () => {
    const fileSubmenu = await Submenu.new({
        text: 'File',
        items: [
            await MenuItem.new({
                id: 'settings',
                text: 'Settings',
                action: () => {
                    console.log('New clicked');
                    goto('/settings');
                },
            }),
            await MenuItem.new({
                id: 'open',
                text: 'Open',
                action: () => {
                    console.log('Open clicked');
                },
            }),
            await MenuItem.new({
                id: 'save_as',
                text: 'Save As...',
                action: () => {
                    console.log('Save As clicked');
                },
            }),
        ],
    });
    const menu = await Menu.new({
        items: [
            fileSubmenu,
            await MenuItem.new({
                id: 'quit',
                text: 'Quit',
                action: () => {
                    console.log('Quit pressed');
                },
            }),
        ],
    });

    await menu.setAsAppMenu();
}