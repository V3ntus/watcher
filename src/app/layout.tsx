import type { Metadata } from 'next'
import { Roboto_Mono } from 'next/font/google'

const mono = Roboto_Mono({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'watcher',
  description: 'Kismetdb wifi and ble snooper',
  icons: {
    icon: '/icon.svg',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={mono.className}>{children}</body>
    </html>
  )
}
