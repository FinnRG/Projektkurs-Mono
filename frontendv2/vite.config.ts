import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import eslintPlugin from "@nabla/vite-plugin-eslint"

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {

  if (mode == 'production') {
    return {
      server: {
        host: '0.0.0.0',
        port: 80,
      },
      plugins: [react()]
    }
  }

  return {plugins: [react(), eslintPlugin()]}
})
