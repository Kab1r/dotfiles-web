module.exports = {
    future: {
        removeDeprecatedGapUtilities: true,
        purgeLayersByDefault: true,
    },
    variants: ['focus', 'hover', 'active', 'responsive'],
    purge: [
        './static/**/*.html'
    ]
}