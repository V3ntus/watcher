import React from "react";
import type {Metadata} from 'next';

import {Roboto_Mono} from 'next/font/google';

import {ColorSchemeScript, MantineProvider, mantineHtmlProps} from '@mantine/core';
import '@mantine/core/styles.css';
import './globals.css';

import {config} from '@fortawesome/fontawesome-svg-core'
import '@fortawesome/fontawesome-svg-core/styles.css'
import Navbar from "@/components/ui/Navbar";

config.autoAddCss = false

const mono = Roboto_Mono({subsets: ['latin']})

export const metadata: Metadata = {
    title: 'watcher',
    description: 'Kismetdb wifi and ble snooper',
    icons: {
        icon: '/icon.svg',
    },
}

export default function RootLayout({children}: { children: React.ReactNode }) {
    return (
        <html lang="en" {...mantineHtmlProps}>
        <head>
            <title>watcher</title>
            <ColorSchemeScript/>
        </head>
        <body className={mono.className}>
        <MantineProvider defaultColorScheme="dark">
            <div className="flex w-full h-full">
                <Navbar/>
                {children}
            </div>
        </MantineProvider>
        </body>
        </html>
    )
}
