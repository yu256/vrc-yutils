import { atom, useAtomValue, useSetAtom } from "jotai";

export const sidebarItems = [
	"Social",
	"Friends",
	"Search",
	"Notification",
	"ChatBox",
	"Settings",
] as const satisfies string[];

const routeAtom = atom<(typeof sidebarItems)[number]>(sidebarItems[1]);

export const useRoute = () => useAtomValue(routeAtom);
export const useSetRoute = () => useSetAtom(routeAtom);
