
        #define BUILD "1648437873-mozjpeg-sys"
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
        #define VERSION "1.0.1"
        #define SIZEOF_SIZE_T 8
        