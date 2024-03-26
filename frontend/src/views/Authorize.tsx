import { Button } from "@/components/ui/button";
import {
	Form,
	FormControl,
	FormField,
	FormItem,
	FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useToast } from "@/components/ui/use-toast";
import fetch from "@/fetch";
import { zodResolver } from "@hookform/resolvers/zod";
import { useCallback, type ReactNode, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";

type Config = {
	authorized: boolean;
	compress_webp: boolean;
};

export default function ({ children }: { children: ReactNode }) {
	const config = fetch<Config>("i", "i");

	if (["localhost", "127.0.0.1", "::1"].includes(location.hostname)) {
		if (config instanceof Error) {
			return (
				<div>
					<div>{config.name}</div>
					<div>{config.message}</div>
				</div>
			);
		}

		if (!config.authorized) return <AuthView />;
	}

	return children;
}

function AuthView() {
	const { toast } = useToast();

	const [resp, setResp] = useState<{
		RequiredAuth: {
			token: string;
			auth_type: string;
		};
	}>();

	const FormSchema = z.object({
		name: z.string().min(1),
		password: z.string().min(1),
		twoFactor: z.string(),
	});

	const form = useForm<z.infer<typeof FormSchema>>({
		resolver: zodResolver(FormSchema),
		defaultValues: {
			name: "",
			password: "",
			twoFactor: "",
		},
	});

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	const onSubmit = useCallback(
		({ name, password, twoFactor }: z.infer<typeof FormSchema>) => {
			type Resp = NonNullable<typeof resp> | "Success";
			const res = window.fetch("auth", {
				method: "POST",
				body: JSON.stringify(
					resp
						? {
								TwoFactor: {
									token: resp.RequiredAuth.token,
									auth_type: resp.RequiredAuth.auth_type,
									two_factor_code: twoFactor,
								},
							}
						: {
								Auth: {
									encoded: `Basic ${btoa(
										`${encodeURIComponent(name)}:${encodeURIComponent(
											password,
										)}`,
									)}`,
								},
							},
				),
				headers: {
					"Content-Type": "application/json",
				},
			});

			res.then((res) => {
				if (res.ok) {
					res.json().then((r: Resp) => {
						if (r !== "Success") {
							setResp(r);
						} else {
							location.reload();
						}
					});
				} else {
					toast({
						title: "Error",
						description: "Invalid credentials",
						variant: "destructive",
					});
				}
			});
		},
		[resp],
	);

	return (
		<div>
			<Form {...form}>
				<form
					onSubmit={form.handleSubmit(onSubmit)}
					className="w-2/3 space-y-6"
				>
					<FormField
						control={form.control}
						name="name"
						render={({ field }) => (
							<FormItem>
								<FormControl>
									<Input
										placeholder="enter your name or email"
										{...field}
										disabled={!!resp}
									/>
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					/>
					<FormField
						control={form.control}
						name="password"
						render={({ field }) => (
							<FormItem>
								<FormControl>
									<Input
										placeholder="enter your password"
										{...field}
										disabled={!!resp}
									/>
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					/>
					<FormField
						control={form.control}
						name="twoFactor"
						render={({ field }) => (
							<FormItem>
								<FormControl>
									<Input
										placeholder="enter your 2fa code"
										{...field}
										disabled={!resp}
									/>
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					/>
					<Button type="submit">Submit</Button>
				</form>
			</Form>
		</div>
	);
}
