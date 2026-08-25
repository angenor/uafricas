<script setup lang="ts">
/**
 * Coordonnées publiques d'un support média (09p), affichées sur la page de la
 * chaîne ou de la station une fois celle-ci validée.
 *
 * Complète : sans remplacer : le bouton « Contacter » de la messagerie interne
 * (US6, FR-046) : celui-ci exige un compte et ne joint que les co-détenteurs
 * inscrits, là où ces coordonnées s'adressent à un visiteur de passage, un
 * annonceur ou un partenaire.
 *
 * Le composant ne rend rien quand aucune coordonnée n'est renseignée : c'est
 * l'API qui omet `contacts`, et rien n'oblige une chaîne à publier les siennes.
 */
import type { ContactsSupport } from '~/composables/useContactsSupport'

const props = defineProps<{
  contacts?: ContactsSupport | null
  /** Nom du support, pour l'intitulé du bloc. */
  nomSupport?: string
}>()

const visible = computed(() => aDesContacts(props.contacts))

/**
 * Une entrée par coordonnée renseignée. Construire la liste ici plutôt que
 * d'empiler cinq `v-if` garde le gabarit lisible et l'ordre d'affichage en un
 * seul endroit : du canal le plus direct au plus formel.
 */
const entrees = computed(() => {
  const c = props.contacts
  if (!c) return []

  return [
    {
      cle: 'whatsapp',
      icone: ['fab', 'whatsapp'] as [string, string],
      libelle: c.whatsapp ?? '',
      lien: lienWhatsapp(c.whatsapp),
      externe: true,
      intitule: 'WhatsApp',
    },
    {
      cle: 'telephone',
      icone: ['fas', 'phone'] as [string, string],
      libelle: c.telephone ?? '',
      lien: lienTelephone(c.telephone),
      externe: false,
      intitule: 'Téléphone',
    },
    {
      cle: 'email',
      icone: ['fas', 'envelope'] as [string, string],
      libelle: c.email ?? '',
      lien: lienEmail(c.email),
      externe: false,
      intitule: 'E-mail',
    },
    {
      cle: 'site_web',
      icone: ['fas', 'globe'] as [string, string],
      libelle: libelleSiteWeb(c.site_web),
      lien: lienSiteWeb(c.site_web),
      externe: true,
      intitule: 'Site web',
    },
    {
      cle: 'adresse',
      icone: ['fas', 'location-dot'] as [string, string],
      libelle: c.adresse ?? '',
      // Une adresse n'est pas cliquable : aucune garantie qu'elle soit
      // géocodable, et un lien de carte mort vaut moins qu'un texte juste.
      lien: null,
      externe: false,
      intitule: 'Adresse',
    },
  ].filter(e => e.libelle.trim() !== '')
})
</script>

<template>
  <section v-if="visible" class="mb-12">
    <h2 class="font-oswald text-xl font-bold text-white mb-4">
      Contacter {{ nomSupport || 'ce média' }}
    </h2>

    <ul class="grid sm:grid-cols-2 gap-3">
      <li v-for="entree in entrees" :key="entree.cle">
        <component
          :is="entree.lien ? 'a' : 'div'"
          :href="entree.lien || undefined"
          :target="entree.lien && entree.externe ? '_blank' : undefined"
          :rel="entree.lien && entree.externe ? 'noopener noreferrer' : undefined"
          class="flex items-start gap-3 rounded-xl border border-white/10 bg-white/5 px-4 py-3 transition-colors"
          :class="entree.lien ? 'hover:border-yellow-400/60 hover:bg-white/10' : ''"
        >
          <span class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-white/10 text-yellow-400">
            <font-awesome-icon :icon="entree.icone" class="w-4 h-4" />
          </span>
          <span class="min-w-0">
            <span class="block text-xs uppercase tracking-wide text-gray-500">{{ entree.intitule }}</span>
            <span class="block text-sm text-gray-200 break-words">{{ entree.libelle }}</span>
          </span>
        </component>
      </li>
    </ul>
  </section>
</template>
