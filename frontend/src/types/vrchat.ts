export type User = {
	id: string;
	location: string | null;
	travelingToLocation: string | null;
	displayName: string;
	userIcon?: string;
	bio: string;
	bioLinks: string[];
	profilePicOverride?: string;
	statusDescription: string;
	currentAvatarImageUrl: string;
	currentAvatarThumbnailImageUrl: string;
	tags: string[];
	developerType: string;
	last_login: string;
	last_platform: string;
	status: "join me" | "active" | "ask me" | "busy" | "offline";
	isFriend: boolean;
	friendKey: string;
};

export type Users = {
	myself: User;
	online: User[];
	web: User[];
	offline: User[];
};
