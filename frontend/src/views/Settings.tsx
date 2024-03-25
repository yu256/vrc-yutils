import { Button } from "@/components/ui/button";
import {
	Form,
	FormControl,
	FormField,
	FormItem,
	FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import type { Config } from "@/types/config";
import { useCallback, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { config as cfg } from "@/atoms";

const FormSchema = z.object({
	alternativeServer: z.object({
		url: z.string().url("URL is not valid"),
		auth: z.string().min(1, "Auth is required"),
	}),
});

export default function () {
	const [config, setConfig] = useState<Config>(cfg);

	const form = useForm<z.infer<typeof FormSchema>>({
		resolver: zodResolver(FormSchema),
		defaultValues: {
			alternativeServer: config.alternativeServer ?? {
				auth: "",
				url: "",
			},
		},
	});

	const onSubmit = useCallback((data: z.infer<typeof FormSchema>) => {
		localStorage.setItem("config", JSON.stringify(data));
		setConfig(data);
	}, []);

	return (
		<div>
			<Form {...form}>
				<form
					onSubmit={form.handleSubmit(onSubmit)}
					className="w-2/3 space-y-6"
				>
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
		</div>
	);
}
