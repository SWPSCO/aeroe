import aspectRatio from '@tailwindcss/aspect-ratio';
import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      fontFamily: {
        content: ['"Afacad Variable"', 'sans-serif'],
        title: ['"Roboto Mono Variable"', 'monospace'],
      },
      colors: {
        light: '#F6F6F6',
        dark: '#181818',
        gray: {
          1: '#E2E2E2',
          2: '#80808020',
          3: '#F1F1F1'
        },
        half: '#808080',
        accent: '#C6F5F9',
        highlight: {
          1: '#F7902E',
          2: '#199FED',
          orange: '#FD4102',
        },
        status: {
          green: '#2DA825',
          gray: '#9B9696',
        },
      },
      dropShadow: {
        'hard': '8px 10px 1px rgba(221, 228, 231, 1)',
      }
    }
  },

  plugins: [typography, forms, containerQueries, aspectRatio]
};
