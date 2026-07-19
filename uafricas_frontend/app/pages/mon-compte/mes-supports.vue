<script setup lang="ts">
/**
 * Les supports (chaînes et stations) que je co-détiens — US5.
 *
 * On devient détenteur de deux façons seulement : par la validation d'une
 * proposition de chaîne ou de station (on en est alors propriétaire), ou par
 * l'acceptation d'une invitation (co-détenteur ou programmateur). Cette page
 * est donc le point d'arrivée des deux parcours, et le seul endroit d'où l'on
 * pilote la grille de programmation et l'équipe d'un support.
 */
import {
  useMediaDetention,
  LIBELLES_ROLE_DETENTEUR,
  DESCRIPTIONS_ROLE_DETENTEUR,
  type DetenteurAPI,
} from '~/composables/useMediaDetention'

definePageMeta({ middleware: 'auth' })

useHead({ title: 'Mes supports médias — UAfricas' })

const { mesSupports, chargement, erreur } = useMediaDetention()
const { listerContenusChaine } = useTelevision()
const { listerContenusStation } = useStationsRadio()

const supports = ref<DetenteurAPI[]>([])

/** Identifiant du support dont le panneau de gestion est déplié (un seul à la fois). */
const gestionOuverte = ref<string | null>(null)

/**
 * Émissions de chaque support, pour le sélecteur de contenu de la grille.
 *
 * Chargées à l'ouverture du panneau et mémorisées : sans elles le sélecteur
 * reste vide et aucun créneau n'est créable — la grille n'a alors rien à
 * programmer (US5). Clé : l'identifiant du support.
 */
const contenusParSupport = ref<Record<string, { id: string, titre: string }[]>>({})

const chargerContenus = async (detenteur: DetenteurAPI) => {
  if (contenusParSupport.value[detenteur.support_id]) return
  const contenus = detenteur.type_support === 'chaine_tv'
    ? await listerContenusChaine(detenteur.support_id)
    : await listerContenusStation(detenteur.support_id)
  contenusParSupport.value[detenteur.support_id] = contenus.map(c => ({
    id: c.id,
    titre: c.title,
  }))
}

const charger = async () => {
  supports.value = await mesSupports()
}

onMounted(charger)

const basculerGestion = (detenteur: DetenteurAPI) => {
  const id = detenteur.id
  gestionOuverte.value = gestionOuverte.value === id ? null : id
  if (gestionOuverte.value === id) chargerContenus(detenteur)
}

const LIBELLES_TYPE_SUPPORT: Record<DetenteurAPI['type_support'], string> = {
  chaine_tv: 'Chaîne de télévision',
  station_radio: 'Station de radio',
}

const iconeType = (type: DetenteurAPI['type_support']): [string, string] =>
  type === 'chaine_tv' ? ['fas', 'tv'] : ['fas', 'radio']

/** La page publique n'existe que si le support a un slug (fiche publiée). */
const lienPublic = (detenteur: DetenteurAPI): string | null => {
  if (!detenteur.support_slug) return null
  return detenteur.type_support === 'chaine_tv'
    ? `/medias/chaines/${detenteur.support_slug}`
    : `/medias/stations/${detenteur.support_slug}`
}

const dateFormatee = (iso: string) =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
    .format(new Date(iso))

const breadcrumbs = [
  { label: 'Mon compte', to: '/mon-compte/profil' },
  { label: 'Mes supports médias', to: undefined },
]
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-4xl mx-auto px-4">
      <nav aria-label="Fil d'Ariane" class="mb-6 text-sm text-gray-500">
        <template v-for="(fil, i) in breadcrumbs" :key="i">
          <NuxtLink v-if="fil.to" :to="fil.to" class="hover:text-custom-chocolat">{{ fil.label }}</NuxtLink>
          <span v-else class="text-gray-900">{{ fil.label }}</span>
          <span v-if="i < breadcrumbs.length - 1" class="mx-2">/</span>
        </template>
      </nav>

      <header class="mb-8">
        <h1 class="font-oswald text-3xl font-bold text-gray-900 mb-2">Mes supports médias</h1>
        <p class="text-gray-500">
          Les chaînes et stations que vous détenez : leur grille de programmation
          et les personnes qui vous accompagnent.
        </p>
      </header>

      <div v-if="erreur" class="mb-6 rounded-lg bg-red-50 border border-red-200 px-4 py-3 text-sm text-red-900">
        {{ erreur }}
      </div>

      <div v-if="chargement" class="flex justify-center py-16">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-yellow-400"></div>
      </div>

      <div v-else-if="supports.length === 0" class="text-center py-16">
        <font-awesome-icon :icon="['fas', 'tv']" class="w-12 h-12 text-gray-300 mb-4" />
        <p class="text-gray-600 mb-1">Vous ne détenez aucun support pour l’instant.</p>
        <p class="text-sm text-gray-400 max-w-md mx-auto">
          On devient détenteur en faisant valider une proposition de chaîne ou de
          station — vous en êtes alors le propriétaire — ou en acceptant
          l’invitation d’un propriétaire.
        </p>
        <div class="mt-5 flex flex-wrap items-center justify-center gap-4 text-sm">
          <NuxtLink
            to="/mon-compte/propositions-medias"
            class="rounded-full bg-custom-chocolat px-5 py-2 font-medium text-white transition-colors hover:bg-custom-chocolat/90"
          >
            Mes propositions de médias
          </NuxtLink>
          <NuxtLink
            to="/mon-compte/invitations-medias"
            class="rounded-full border border-gray-300 bg-white px-5 py-2 font-medium text-gray-700 transition-colors hover:bg-gray-50"
          >
            Mes invitations
          </NuxtLink>
        </div>
      </div>

      <ul v-else class="space-y-4">
        <li
          v-for="detenteur in supports"
          :key="detenteur.id"
          class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden"
        >
          <div class="p-5">
            <div class="flex items-start gap-4">
              <!-- Visuel du support, ou repli iconographique selon le type. -->
              <div class="shrink-0 h-16 w-16 rounded-lg bg-gray-100 overflow-hidden flex items-center justify-center">
                <img
                  v-if="detenteur.support_image"
                  :src="detenteur.support_image"
                  :alt="detenteur.support_nom || 'Support'"
                  class="h-full w-full object-cover"
                >
                <font-awesome-icon v-else :icon="iconeType(detenteur.type_support)" class="w-6 h-6 text-gray-400" />
              </div>

              <div class="min-w-0 flex-1">
                <h2 class="font-semibold text-gray-900 truncate">
                  {{ detenteur.support_nom || 'Support sans nom' }}
                </h2>
                <div class="mt-1.5 flex flex-wrap items-center gap-2">
                  <span class="rounded-full bg-gray-100 px-3 py-1 text-xs font-medium text-gray-700">
                    {{ LIBELLES_TYPE_SUPPORT[detenteur.type_support] }}
                  </span>
                  <span
                    class="rounded-full bg-amber-100 px-3 py-1 text-xs font-bold uppercase tracking-wide text-amber-800"
                    :title="DESCRIPTIONS_ROLE_DETENTEUR[detenteur.role]"
                  >
                    {{ LIBELLES_ROLE_DETENTEUR[detenteur.role] }}
                  </span>
                </div>
                <p class="mt-2 text-xs text-gray-400">
                  Détenteur depuis le {{ dateFormatee(detenteur.designe_at) }}
                </p>
              </div>
            </div>

            <div class="mt-4 flex flex-wrap items-center gap-3">
              <NuxtLink
                v-if="lienPublic(detenteur)"
                :to="lienPublic(detenteur)!"
                class="inline-flex items-center gap-2 rounded-full border border-gray-300 bg-white px-4 py-1.5 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
              >
                <font-awesome-icon :icon="['fas', 'eye']" class="w-3.5 h-3.5" />
                Voir la page publique
              </NuxtLink>

              <button
                type="button"
                class="inline-flex cursor-pointer items-center gap-2 rounded-full px-4 py-1.5 text-sm font-medium transition-colors"
                :class="gestionOuverte === detenteur.id
                  ? 'bg-gray-800 text-white'
                  : 'bg-custom-chocolat text-white hover:bg-custom-chocolat/90'"
                :aria-expanded="gestionOuverte === detenteur.id"
                @click="basculerGestion(detenteur)"
              >
                <font-awesome-icon :icon="['fas', 'sliders']" class="w-3.5 h-3.5" />
                {{ gestionOuverte === detenteur.id ? 'Fermer la gestion' : 'Gérer' }}
              </button>
            </div>
          </div>

          <!-- Panneau de gestion déplié en place : pas de navigation, on reste
               dans la liste de ses supports. -->
          <div v-if="gestionOuverte === detenteur.id" class="border-t border-gray-100 bg-gray-50/60 p-5 space-y-8">
            <section>
              <h3 class="font-oswald text-lg font-bold text-gray-900 mb-3">Grille de programmation</h3>
              <!-- Les émissions du support, chargées à l'ouverture du panneau :
                   c'est la seule source du sélecteur de contenu, sans laquelle
                   aucun créneau n'est créable. -->
              <MediaGrilleProgrammation
                :type-support="detenteur.type_support"
                :support-id="detenteur.support_id"
                :mon-role="detenteur.role"
                :contenus="contenusParSupport[detenteur.support_id] ?? []"
                :modifiable="true"
              />
              <p
                v-if="(contenusParSupport[detenteur.support_id] ?? []).length === 0"
                class="mt-2 text-sm text-gray-500"
              >
                Ce support n'a encore aucune émission publiée : publiez-en une
                avant de bâtir sa grille.
              </p>
            </section>

            <section>
              <h3 class="font-oswald text-lg font-bold text-gray-900 mb-3">
                Demandes reçues
              </h3>
              <p class="mb-3 text-sm text-gray-600">
                Les idées de contenu déposées par les visiteurs et les demandes
                d'animation adressées à ce support.
              </p>
              <MediaDemandesEngagementSupport
                :type-support="detenteur.type_support"
                :support-id="detenteur.support_id"
                :mon-role="detenteur.role"
              />
            </section>

            <section>
              <h3 class="font-oswald text-lg font-bold text-gray-900 mb-3">Équipe du support</h3>
              <MediaGestionCoDetenteurs
                :type-support="detenteur.type_support"
                :support-id="detenteur.support_id"
                :mon-role="detenteur.role"
              />
            </section>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>
