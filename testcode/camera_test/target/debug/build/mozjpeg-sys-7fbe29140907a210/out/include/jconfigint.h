
        #define BUILD "1647971495-mozjpeg-sys"
        #ifndef INLINE
            #if defined(__GNUC__)
                #define INLINE inline __attribute__((always_inline))
            #elif defined(_MSC_VER)
                #define INLINE __forceinline
            #else
                #define INLINE inline
            #endif
        #endif
        #define PACKAGE_NAME "mozjpeg-sys"
        #define VERSION "0.10.13"
        #define SIZEOF_SIZE_T 8
        