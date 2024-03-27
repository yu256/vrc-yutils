import { Button } from "@/components/ui/button";
import {
	Form,
	FormControl,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useCallback, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { config as cfg, proxyUrl } from "@/atoms";
import { useToast } from "@/components/ui/use-toast";
import { ToastAction } from "@radix-ui/react-toast";

const FormSchema = z.object({
	alternativeServer: z.object({
		url: z.string().url("URL is not valid"),
		auth: z.string().min(1, "Auth is required"),
	}),
});

export default function () {
	const { toast } = useToast();
	const reloadToast = () =>
		toast({
			title: "Reload",
			description: "Reload to reflect changes",
			action: (
				<ToastAction
					altText="
            Reload
            "
					onClick={() => window.location.reload()}
				>
					Reload
				</ToastAction>
			),
		});

	return (
		<div className="grid gap-3">
			<AlternativeServer reloadToast={reloadToast} />
			<Auth toast={toast} />
			<ProxyUrl reloadToast={reloadToast} />
		</div>
	);
}

function AlternativeServer({ reloadToast }: { reloadToast: () => void }) {
	const [config, setConfig] = useState(cfg);

	const form = useForm<z.infer<typeof FormSchema>>({
		resolver: zodResolver(FormSchema),
		defaultValues: {
			alternativeServer: config.alternativeServer ?? {
				auth: "",
				url: "",
			},
		},
	});

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	const onSubmit = useCallback((data: z.infer<typeof FormSchema>) => {
		localStorage.setItem("config", JSON.stringify(data));
		setConfig(data);
		reloadToast();
	}, []);

	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="w-2/3 space-y-6">
				<FormLabel className="font-bold">外部サーバー</FormLabel>
				<FormField
					control={form.control}
					name="alternativeServer.url"
					render={({ field }) => (
						<FormItem>
							<FormControl>
								<Input placeholder="https://..." {...field} />
							</FormControl>
							<FormMessage />
						</FormItem>
					)}
				/>
				<FormField
					control={form.control}
					name="alternativeServer.auth"
					render={({ field }) => (
						<FormItem>
							<FormControl>
								<Input placeholder="Auth" {...field} />
							</FormControl>
							<FormMessage />
						</FormItem>
					)}
				/>
				<Button type="submit">Submit</Button>
			</form>
		</Form>
	);
}

function Auth({ toast }: { toast: ReturnType<typeof useToast>["toast"] }) {
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

	const isTempAuthorized = !resp;

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
							toast({
								title: "requiresTwoFactorAuth",
								description: "TwoFactor required",
							});
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
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="w-2/3 space-y-6">
				<FormLabel className="font-bold">認証</FormLabel>
				<FormField
					control={form.control}
					name="name"
					render={({ field }) => (
						<FormItem>
							<FormControl>
								<Input
									placeholder="enter your name or email"
									{...field}
									disabled={!isTempAuthorized}
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
									disabled={!isTempAuthorized}
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
									disabled={isTempAuthorized}
								/>
							</FormControl>
							<FormMessage />
						</FormItem>
					)}
				/>
				<Button type="submit">Submit</Button>
			</form>
		</Form>
	);
}

function ProxyUrl({ reloadToast }: { reloadToast: () => void }) {
	const formSchema = z.object({
		url: z
			.string()
			.url("URL is not valid")
			.endsWith("/", "URL must end with /"),
	});

	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
		defaultValues: {
			url: proxyUrl,
		},
	});

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	const onSubmit = useCallback((data: z.infer<typeof formSchema>) => {
		localStorage.setItem("proxyUrl", data.url);
		reloadToast();
	}, []);

	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="w-2/3 space-y-6">
				<FormLabel className="font-bold">Image Proxy URL</FormLabel>
				<FormField
					control={form.control}
					name="url"
					render={({ field }) => (
						<FormItem>
							<FormControl>
								<Input placeholder="https://.../" {...field} />
							</FormControl>
							<FormMessage />
						</FormItem>
					)}
				/>
				<Button type="submit">Submit</Button>
			</form>
		</Form>
	);
}
