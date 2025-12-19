import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: '/convert',
        destination: `${process.env.BACKEND_URL || 'http://localhost:8080'}/convert`,
      },
    ];
  },
};

export default nextConfig;
