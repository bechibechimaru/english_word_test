import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    host: true,
    port: 5173,
    allowedHosts: [
      "ec2-54-178-56-216.ap-northeast-1.compute.amazonaws.com"
    ]
  }
})
