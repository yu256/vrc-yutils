import { cn } from "@/lib/utils";
import type { User } from "@/types/vrchat";

const defaultAvatar =
	"https://api.vrchat.cloud/api/1/image/file_0e8c4e32-7444-44ea-ade4-313c010d4bae/1/256";

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
			src={
				user.currentAvatarThumbnailImageUrl === defaultAvatar
					? user.profilePicOverride || user.userIcon
					: user.currentAvatarThumbnailImageUrl || defaultAvatar
			}
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
								? "bg-yellow-500"
								: user.status === "offline"
									? "bg-black"
									: user.status === "join me"
										? "bg-blue-500"
										: "bg-red-500",
					)}
				/>
			</div>
		)}
	</div>
);

export { VrcAvatar };
