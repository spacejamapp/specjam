import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async redirects() {
    return [
      {
        source: "/",
        destination: "/api/download",
        permanent: true,
      },
    ];
  },
};

export default nextConfig;
