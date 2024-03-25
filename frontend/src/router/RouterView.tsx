import Settings from "@/views/Settings";
import { useRoute } from "./router";
import Friends from "@/views/Friends";

export default function () {
	const route = useRoute();

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
