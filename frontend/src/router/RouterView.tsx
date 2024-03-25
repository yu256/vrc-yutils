import Settings from "@/views/Settings";
import type { sidebarItems } from "./router";
import Friends from "@/views/Friends";

export default function ({
	route,
}: {
	route: (typeof sidebarItems)[number];
}) {
	switch (route) {
		// case "Social":
		// 	return <Social />;
		case "Friends":
			return <Friends />;
		// case "ChatBox":
		// 	return <ChatBox />;
		// case "Search":
		// 	return <Search />;
		case "Settings":
			return <Settings />;
	}
}
