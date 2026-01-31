import { library, config } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import {
  faArrowRight,
  faArrowDown,
  faTv,
  faEnvelope,
  faPhone,
  faBars,
  faTimes,
  faChevronDown,
  faChevronUp,
  faUser,
  faSearch,
  faHome,
  faGlobe,
  faUsers,
  faHandshake,
  faLightbulb,
  faBook,
  faGraduationCap,
  faBriefcase,
  faMapMarkerAlt,
  faCalendar,
  faPlay,
  faPause,
  faVolumeUp,
  faVolumeMute,
} from '@fortawesome/free-solid-svg-icons'
import {
  faFacebook,
  faTwitter,
  faLinkedin,
  faInstagram,
  faYoutube,
  faWhatsapp,
} from '@fortawesome/free-brands-svg-icons'

// Prevent auto CSS injection
config.autoAddCss = false

// Add icons to library
library.add(
  faArrowRight,
  faArrowDown,
  faTv,
  faEnvelope,
  faPhone,
  faBars,
  faTimes,
  faChevronDown,
  faChevronUp,
  faUser,
  faSearch,
  faHome,
  faGlobe,
  faUsers,
  faHandshake,
  faLightbulb,
  faBook,
  faGraduationCap,
  faBriefcase,
  faMapMarkerAlt,
  faCalendar,
  faPlay,
  faPause,
  faVolumeUp,
  faVolumeMute,
  faFacebook,
  faTwitter,
  faLinkedin,
  faInstagram,
  faYoutube,
  faWhatsapp
)

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.component('font-awesome-icon', FontAwesomeIcon)
})
