/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // Transpile Three.js and VRM packages for proper bundling
  transpilePackages: ['three', '@pixiv/three-vrm'],
  webpack: (config) => {
    // Ensure proper handling of Three.js modules
    config.externals = config.externals || {};

    return config;
  },
}

module.exports = nextConfig
