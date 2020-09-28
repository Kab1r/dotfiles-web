import './inject-tailwind.css';
import './inject-fontawesome.css';

import ("../pkg").then(module => {
    module.run_app();
});