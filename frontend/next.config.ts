import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: '/convert',
        destination: 'http://localhost:8080/convert',
      },
    ];
  },
};

export default nextConfig;
