import type { Config } from 'tailwindcss'

const toVar = (shade: string) => `oklch(var(--color-gray-${shade}))`

const config: Config = {
  content: [
    './src/**/*.{html,js,ts,svelte,vue}',
    './index.html',
  ],
  theme: {
    extend: {
      colors: {
        gray: {
          50: toVar('50'),
          100: toVar('100'),
          200: toVar('200'),
          300: toVar('300'),
          400: toVar('400'),
          500: toVar('500'),
          600: toVar('600'),
          700: toVar('700'),
          800: toVar('800'),
          900: toVar('900'),
          950: toVar('950'),
        },
      },
    },
  },
}

export default config
