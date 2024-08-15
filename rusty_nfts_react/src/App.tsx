import React, { useState, useEffect } from "react";
import init, { apply_filter } from "../public/pkg";
import "./index.css";

function App() {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [filteredImage, setFilteredImage] = useState<string | null>(null);
  const [filterType, setFilterType] = useState<string>("grayscale");
  const [wasmInitialized, setWasmInitialized] = useState(false);

  useEffect(() => {
    const initializeWasm = async () => {
      try {
        await init({});
        setWasmInitialized(true);
      } catch (error) {
        console.error("Failed to initialize WebAssembly module", error);
      }
    };

    initializeWasm();
  }, []);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files.length > 0) {
      setSelectedFile(event.target.files[0]);
      setFilteredImage(null);
    }
  };

const handleFilterApply = async () => {
  if (!wasmInitialized) {
    console.warn("WebAssembly module not yet initialized");
    return; 
  }

  if (selectedFile) {
    // Create a new FileReader to read the file's data
    // FileReader is a built-in JavaScript object that allows web applications to asynchronously read the contents of files
    const reader = new FileReader();

    // onload => Event handler that is called when the file reading operation is successfully completed
    reader.onload = async (e: ProgressEvent<FileReader>) => {
      const result = e.target?.result; // Get the result of the file reading operation

      // Check if the result exists and is not a string (it should be binary data)
      if (result && typeof result !== "string") {
        // We used 'u8' in Rust (perfect for representing the pixel values) so create an array of unsigned 8-bit integers from the 'result'
        const imgData = new Uint8Array(result);

        // Apply the selected filter using the WebAssembly function `apply_filter`
        // This function takes the image data and the selected filter type as arguments
        const filteredData = apply_filter(imgData, filterType);

        // Create a Blob (Binary Large Object) from the filtered data, specifying that it's a PNG image
        // Why do we need a blob? A Uint8Array is still just raw binary data
        // So, for it to be downloadable or viewable image file, it needs to be encapsulated in a blob. 
        // The type is the MIME type -> tells the browser how to interpret the data
        const filteredBlob = new Blob([filteredData], { type: "image/png" });

        // Generate a URL for the Blob and set it as the filtered image to display it
        setFilteredImage(URL.createObjectURL(filteredBlob));
      }
    };

    // Start reading the selected file as an ArrayBuffer (binary data)
    // reader.onload is async (so waits to be triggered) and set up before 'readAsArrayBuffer' so it is guaranteed to be ready when the file is read
    reader.readAsArrayBuffer(selectedFile);
  }
};

  const handleReset = () => {
    setSelectedFile(null);
    setFilteredImage(null);
  };

  const handleDownload = () => {
    if (filteredImage) {
      const link = document.createElement("a");
      link.href = filteredImage;
      link.download = "rusty_nft.png";
      link.click();
    }
  };

  return (
    <div className="flex flex-col items-center w-screen h-screen overflow-hidden bg-center bg-5 bg-cover">
      <h2 className="text-6xl font-bold text-center mb-2 pb-7 font-lacquer p-2 w-full bg-gray-100 shadow-gray-200 shadow-2xl">
        Welcome to Rusty NFTs
      </h2>
      <h4 className="text-2xl italic mb-6 -mt-10 -ml-72 font-cang">
        Make NFTs from your favourite images!
      </h4>
      <div className="flex flex-col items-center bg-gray-100 pt-3 pb-3 p-5 rounded-lg border border-gray-200 shadow-md shadow-gray-400 w-1/4">
        <div className="flex items-center gap-5">
          <label className="text-center font-lacquer cursor-pointer bg-blue-500 text-black p-2 rounded-md hover:bg-blue-600 transform transition">
            Choose File
            <input type="file" onChange={handleFileChange} className="hidden" />
          </label>
          {selectedFile && (
            <p className="text-lg text-gray-700 font-lacquer">
              {selectedFile.name}
            </p>
          )}
        </div>
        <div className="flex ml-2 mt-2">
          <label htmlFor="filter-select" className="font-doodle text-3xl">
            Filter:
          </label>
          <select
            id="filter-select"
            onChange={(e) => setFilterType(e.target.value)}
            className="bg-gray-100 border border-gray-300 rounded-md ml-2 font-cang text-3xl text-center"
          >
            <option value="grayscale">Grayscale</option>
            <option value="blur">Blur</option>
            <option value="huerotate">Hue Rotate</option>
            <option value="invert">Invert Colors</option>
            <option value="sepia">Sepia</option>
            <option value="pixelate">Pixelate</option>
            <option value="emboss">Emboss</option>
            <option value="sharpen">Sharpen</option>
            <option value="posterize">Posterize</option>
          </select>
        </div>
        <div className="flex justify-center w-full gap-5 mt-2">
          <button
            onClick={handleFilterApply}
            className="bg-blue-500 text-black p-1 font-marker rounded-md text-opacity-85 text-1xl w-5/12 hover:bg-blue-600 transform transition duration-700 hover:scale-110 hover:text-white"
          >
            Apply
          </button>
          <button
            onClick={handleReset}
            className="bg-red-500 text-black p-1 font-marker rounded-md text-1xl w-5/12 hover:bg-red-600 transform transition duration-700 hover:scale-110 hover:text-white"
          >
            Reset
          </button>
        </div>
      </div>
      {selectedFile && (
      <div className="flex justify-center gap-5 mt-3 bg-gray-100 p-5 pt-2 rounded-lg shadow-lg shadow-gray-500">
          <div className="flex flex-col items-center">
            <h2 className="font-marker mb-1">Original Image:</h2>
            <img
              src={URL.createObjectURL(selectedFile)}
              alt="Original"
              className="max-w-9/12 max-h-60 rounded-md shadow-lg shadow-gray-400"
            />
          </div>
        {filteredImage && (
          <div className="flex flex-col items-center">
            <h2 className="font-marker mb-1">Your Rusty NFT:</h2>
            <img
              src={filteredImage}
              alt="Filtered"
              className="max-w-9/12 max-h-60 rounded-md shadow-lg shadow-gray-400"
            />
          </div>
        )}
        </div>
      )}
      {filteredImage && (
        <button
          onClick={handleDownload}
          className="bg-green-500 text-black-100 mt-2 p-1 font-marker rounded-md text-opacity-85 text-2xl w-1/5 hover:bg-green-600 hover:text-white transform transition duration-700 hover:scale-105"
        >
          Download Image
        </button>
      )}
    </div>
  );
}

export default App;
