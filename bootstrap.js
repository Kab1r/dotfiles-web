import './inject-tailwind.css'

import ("./pkg").then(module => {
    module.run_app();
});