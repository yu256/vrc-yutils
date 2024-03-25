import { useState } from "react";

export const useRoute = () =>
	useState<(typeof sidebarItems)[number]>(sidebarItems[1]);

export const sidebarItems = [
	"Social",
	"Friends",
	"Search",
	"Notification",
	"ChatBox",
	"Settings",
] as const satisfies string[];
