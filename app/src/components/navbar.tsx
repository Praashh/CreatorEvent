"use client"
import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { Search } from 'lucide-react';

const Navbar = () => {
  const [isScrolled, setIsScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 0);
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  return (
    <nav className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
      isScrolled ? 'bg-black bg-opacity-50 backdrop-filter backdrop-blur-lg' : 'bg-black'
    }`}>
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <Link href="/" className="flex-shrink-0 mr-8"> {/* Added mr-8 for more space */}
              <div className="relative w-16 h-16">
                <Image
                  src="/logo.png"
                  alt="Logo"
                  fill
                  className="object-contain"
                  priority
                />
              </div>
            </Link>
            <div className="hidden md:flex space-x-6"> {/* Increased space-x-4 to space-x-6 */}
              <Link href="/" className="text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium">
                Home
              </Link>
              <Link href="/events" className="text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium">
                Events
              </Link>
            </div>
          </div>
          <div className="flex-1 max-w-md px-4"> {/* Reduced max-w-xl to max-w-md */}
            <div className="relative">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <Search className="h-5 w-5 text-gray-400" />
              </div>
              <input
                type="text"
                className="block w-full pl-10 pr-3 py-2 border border-transparent rounded-full leading-5 bg-gray-700 text-gray-300 placeholder-gray-400 focus:outline-none focus:bg-white focus:text-gray-900 sm:text-sm transition duration-150 ease-in-out"
                placeholder="Search events, artists..."
              />
            </div>
          </div>
          <div className="flex items-center">
            <button className="ml-4 px-4 py-2 rounded-md text-sm font-medium text-black bg-[#DEFF58] hover:bg-opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white">
              Sign Up
            </button>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;