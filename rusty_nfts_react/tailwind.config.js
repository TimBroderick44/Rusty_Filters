/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        barrio: ["Barrio", "sans-serif"],
        cang: ["Long Cang", "sans-serif"],
        lacquer: ["Lacquer", "sans-serif"],
        doodle: ["Rubik Doodle Shadow", "sans-serif"],
        marker: ["Permanent Marker", "sans-serif"],
      },
      backgroundImage: {
        1: "url('/src/assets/1.jpg')", 
        2: "url('/src/assets/2.jpg')",
        3: "url('/src/assets/3.gif')",
        4: "url('/src/assets/4.png')",
        5: "url('/src/assets/5.png')",
      },
    },
  },
  plugins: [],
};

