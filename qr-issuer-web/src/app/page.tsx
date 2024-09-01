"use client";

import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { cn } from "@/lib/utils";
import { add, format, getUnixTime, isPast } from "date-fns";
import { CalendarIcon } from "lucide-react";
import { useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";

type FormData = {
  name: string;
  time: string;
  date: Date;
};

export default function Home() {
  const [qr, setQr] = useState<Blob | null>(null);

  const form = useForm<FormData>({
    defaultValues: {
      name: "",
      time: format(new Date(), "HH:mm"),
      date: new Date(),
    },
    shouldUseNativeValidation: true,
  });

  const onSubmit: SubmitHandler<FormData> = async (data) => {
    const payload = {
      user: data.name,
      expiry: getUnixTime(
        add(data.date, {
          hours: parseInt(data.time.split(":")[0]),
          minutes: parseInt(data.time.split(":")[1]),
        }),
      ),
    };

    const qrPng = await fetch("http://localhost:8080/", {
      method: "POST",
      body: JSON.stringify(payload),
      headers: {
        "Content-Type": "application/json",
      },
    });

    const qrBlob = await qrPng.blob();
    setQr(qrBlob);
  };

  return (
    <main className="flex min-h-screen flex-col p-24 container gap-12 mx-auto">
      <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
        Generate Door Code
      </h1>
      <div className="flex flex-auto flex-row gap-12">
        <div className="flex-auto">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Recipient Name</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="Enter name of receipient"
                        type="text"
                        required
                        {...field}
                      />
                    </FormControl>
                    <FormDescription />
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="time"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Expiry Time</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="Enter expiry time"
                        type="time"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription />
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="date"
                render={({ field }) => (
                  <FormItem className="flex flex-col">
                    <FormLabel>Expiry Date</FormLabel>
                    <Popover>
                      <PopoverTrigger asChild>
                        <FormControl>
                          <Button
                            variant={"outline"}
                            className={cn(
                              "pl-3 text-left font-normal",
                              !field.value && "text-muted-foreground",
                            )}
                          >
                            {field.value ? (
                              format(field.value, "PPP")
                            ) : (
                              <span>Pick a date</span>
                            )}
                            <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                          </Button>
                        </FormControl>
                      </PopoverTrigger>
                      <PopoverContent className="w-auto p-0" align="start">
                        <Calendar
                          mode="single"
                          selected={field.value}
                          onSelect={field.onChange}
                          disabled={(date) => isPast(add(date, { days: 1 }))}
                          initialFocus
                        />
                      </PopoverContent>
                    </Popover>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <Button variant="outline" type="submit">
                Generate
              </Button>
            </form>
          </Form>
        </div>
        <div>
          <Card>
            <CardHeader>
              <CardTitle>Access QR</CardTitle>
            </CardHeader>
            <CardContent className="flex items-center justify-center p-12">
              <div className="bg-zinc-800 size-64 rounded-md">
                {qr ? <img src={URL.createObjectURL(qr)} /> : ""}
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </main>
  );
}
