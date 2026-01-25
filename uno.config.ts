import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      // True Black background for maximum contrast with video
      kenichi: {
        bg: '#000000',
        panel: '#121212',     // Primary containers
        surface: '#1E1E1E',   // Hover states / Inner containers
        border: '#2A2A2A',    // Subtle separators
        header: '#181818',    // Panel headers
      },
      brand: {
        accent: '#00C4CC',    // Professional Cyan
        gold: '#FFD700',      // Warning/Performance notifications
      },
      ui: {
        muted: '#888888',
        text: '#E0E0E0',
      }
    },
    // Typography scaling for high-density UX
    fontSize: {
      'ui-small': ['11px', '14px'],
      'ui-base': ['13px', '18px'],
      'ui-bold': ['13px', '18px'],
    }
  },
  shortcuts: [
    // Standard Panel styling
    ['nle-panel', 'bg-kenichi-panel border-kenichi-border border-solid'],
    ['panel-header', 'h-32px px-3 flex items-center bg-kenichi-header border-b border-kenichi-border text-ui-small font-bold uppercase tracking-wider text-ui-muted select-none'],

    // Custom Input styling for the Inspector
    ['nle-input', 'bg-black border border-kenichi-border rounded-2px text-ui-small text-ui-text focus:border-brand-accent outline-none transition-colors'],

    // Iconic Buttons
    ['nle-icon-btn', 'p-1 hover:bg-kenichi-surface rounded-4px text-ui-muted hover:text-white transition-all cursor-pointer'],
    ['nle-button-primary', 'bg-brand-accent text-black font-bold text-ui-small px-4 py-1.5 rounded-2px hover:bg-opacity-90 active:scale-95 transition-all'],
  ],
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      cdn: 'https://esm.sh/',
    }),
  ],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
  // Global Styles for Scrollbars and Core Resets
  preflights: [
    {
      getCSS: () => `
        :root {
          background-color: transparent; /* TRANSPARENT for WGPU "Hole Punch" */
          color: #E0E0E0;
          font-family: 'Inter', system-ui, -apple-system, sans-serif;
          overflow: hidden; /* Prevent window-level scroll */
        }

        /* High-Density Custom Scrollbars */
        ::-webkit-scrollbar {
          width: 4px;
          height: 4px;
        }

        ::-webkit-scrollbar-track {
          background: #121212;
        }

        ::-webkit-scrollbar-thumb {
          background: #333;
          border-radius: 10px;
        }

        ::-webkit-scrollbar-thumb:hover {
          background: #00C4CC;
        }

        /* Remove Arrows from Number Inputs (for Scrubbable inputs) */
        input::-webkit-outer-spin-button,
        input::-webkit-inner-spin-button {
          -webkit-appearance: none;
          margin: 0;
        }
      `,
    },
  ],
});
