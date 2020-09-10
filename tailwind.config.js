module.exports = {
    future: {
        removeDeprecatedGapUtilities: true,
        purgeLayersByDefault: true,
    },
    variants: ['focus', 'hover', 'active', 'responsive'],
    purge: [
        './src/**/*.rs',
        './static/**/*.html'
    ]
}