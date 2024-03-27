import { proxyUrl } from "@/atoms";
import { cn } from "@/lib/utils";
import type { User } from "@/types/vrchat";

const defaultAvatar =
	"https://api.vrchat.cloud/api/1/image/file_0e8c4e32-7444-44ea-ade4-313c010d4bae/1/256";

function getSrc(
	user: Pick<
		Partial<User>,
		"currentAvatarThumbnailImageUrl" | "profilePicOverride" | "userIcon"
	>,
) {
	const url =
		user.currentAvatarThumbnailImageUrl === defaultAvatar
			? user.profilePicOverride || user.userIcon
			: user.currentAvatarThumbnailImageUrl || defaultAvatar;
	return proxyUrl ? `${proxyUrl}?url=${url}&w=256&q=75` : url;
}

const VrcAvatar = ({
	user,
	className = "",
	showStatus = true,
}: {
	user: Pick<
		User,
		"profilePicOverride" | "userIcon" | "displayName" | "status"
	> &
		Pick<Partial<User>, "currentAvatarThumbnailImageUrl">;
	className?: string;
	showStatus?: boolean;
}) => (
	<div className="relative inline-block">
		<img
			decoding="async"
			className={cn(
				"w-full h-full rounded-full object-cover aspect-square p-2",
				className,
			)}
			src={getSrc(user)}
			alt={user.displayName}
		/>
		{showStatus && (
			<div className="absolute bottom-0 left-0">
				<div
					className={cn(
						"w-4 h-4 rounded-full",
						user.status === "active"
							? "bg-green-500"
							: user.status === "ask me"
								? "bg-orange-500"
								: user.status === "offline"
									? "bg-black"
									: user.status === "join me"
										? "bg-blue-500"
										: "bg-amber-900",
					)}
				/>
			</div>
		)}
	</div>
);

export { VrcAvatar };
