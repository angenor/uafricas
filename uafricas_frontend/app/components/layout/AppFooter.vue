<template>
  <footer class="bg-white text-gray-800">
    <div class="container mx-auto px-6 md:px-12 py-10">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12">

        <!-- Logo -->
        <div class="flex flex-col items-center md:items-start">
          <NuxtLink to="/">
            <img class="h-24 sm:h-28 lg:h-32" src="/logos/logo_uafracas.png" alt="AfricanS Logo" />
          </NuxtLink>
          <p class="mt-3 text-sm text-gray-500 text-center md:text-left max-w-xs">
            Afrique, une Nation pour le développement durable.
          </p>
        </div>

        <!-- Liens rapides -->
        <div>
          <h3 class="text-custom-chocolat font-semibold text-lg mb-4 font-display">Liens rapides</h3>
          <ul class="space-y-2.5">
            <li v-for="lien in liensRapides" :key="lien.to">
              <NuxtLink
                :to="lien.to"
                class="text-sm text-gray-600 hover:text-custom-green transition-colors flex items-center gap-2"
              >
                <font-awesome-icon :icon="lien.icone" class="w-3.5 h-3.5 text-custom-chocolat/60" />
                {{ lien.label }}
              </NuxtLink>
            </li>
          </ul>
        </div>

        <!-- Contact -->
        <div>
          <h3 class="text-custom-chocolat font-semibold text-lg mb-4 font-display">Contact</h3>
          <div class="space-y-3 mb-6">
            <a href="mailto:uafricas@gmail.com" class="flex items-center gap-2.5 text-sm text-gray-600 hover:text-custom-green transition-colors">
              <font-awesome-icon icon="fa-solid fa-envelope" class="w-4 h-4 text-custom-chocolat/60" />
              uafricas@gmail.com
            </a>
            <!-- Le téléphone affichait « 00 00 00 00 00 ». Un numéro de
                 remplissage n'informe pas, il induit en erreur : quelqu'un
                 finit par le composer. Il reviendra le jour où il existera. -->
          </div>

          <!-- Réseaux sociaux. `href="#"` remontait en haut de page : ce
               n'était pas un lien, c'était un piège. La rangée entière
               disparaît tant qu'aucune adresse n'est renseignée. -->
          <div v-if="reseauxSociaux.length" class="flex gap-2.5">
            <a
              v-for="reseau in reseauxSociaux"
              :key="reseau.nom"
              :href="reseau.url"
              target="_blank"
              rel="noopener noreferrer"
              :title="reseau.nom"
              :aria-label="`AfricanS sur ${reseau.nom}`"
              class="w-9 h-9 rounded-full flex items-center justify-center text-white transition-opacity hover:opacity-80"
              :class="reseau.bg"
            >
              <font-awesome-icon :icon="['fab', reseau.icone]" class="w-4 h-4" />
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- Copyright -->
    <div class="text-center text-white bg-custom-chocolat py-2 text-sm">
      &copy; Copyright {{ anneeCourante }}
    </div>
  </footer>
</template>

<script setup lang="ts">
// Année de copyright dynamique (constat #13 de l'audit)
const anneeCourante = new Date().getFullYear()

const liensRapides = [
  // La page de présentation n'est plus la racine : sans ce lien, elle ne serait
  // atteignable qu'en tapant son adresse.
  { label: 'Découvrir AfricanS', to: '/decouvrir', icone: 'fa-solid fa-earth-africa' },
  { label: 'Notre Mission', to: '/a-propos/mission', icone: 'fa-solid fa-rocket' },
  { label: 'Gouvernance', to: '/universite/gouvernance', icone: 'fa-solid fa-landmark' },
  { label: 'Nos Partenaires', to: '/a-propos/partenaires', icone: 'fa-solid fa-handshake' },
  { label: 'FAQ', to: '/a-propos/faq', icone: 'fa-solid fa-circle-info' },
  { label: 'Contactez-nous', to: '/a-propos/contact', icone: 'fa-solid fa-envelope' },
  { label: 'Devenir Partenaire', to: '/devenir-partenaire', icone: 'fa-solid fa-user-plus' },
]

/**
 * Réseaux sociaux de la plateforme.
 *
 * Les cinq boutons n'avaient AUCUN `href` : cinq pastilles colorées qui ne
 * menaient nulle part, et qu'un lecteur d'écran n'annonçait même pas comme
 * des liens. Chacun porte désormais son adresse, et seuls ceux qui en ont
 * une sont affichés — renseigner `url` suffit à rallumer un réseau.
 */
const reseauxSociaux = [
  { nom: 'Facebook', icone: 'facebook', bg: 'bg-blue-600', url: '' },
  { nom: 'Twitter', icone: 'twitter', bg: 'bg-sky-500', url: '' },
  { nom: 'LinkedIn', icone: 'linkedin', bg: 'bg-blue-700', url: '' },
  { nom: 'Instagram', icone: 'instagram', bg: 'bg-pink-600', url: '' },
  { nom: 'YouTube', icone: 'youtube', bg: 'bg-red-600', url: '' },
].filter(r => r.url)
</script>
