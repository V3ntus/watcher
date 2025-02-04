import type { Metadata } from 'next'
import { Roboto_Mono } from 'next/font/google'

const mono = Roboto_Mono({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'wasm-next',
  description: 'wasm-next',
  icons: {
    icon: '/icon.ico',
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
