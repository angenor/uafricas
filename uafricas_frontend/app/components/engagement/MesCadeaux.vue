<script setup lang="ts">
/**
 * Mes cadeaux reçus et offerts — Tailwind v4 pur.
 *
 * Le **montant n'est affiché que sur le sens « offerts »** : l'offreur a le
 * droit de savoir ce qu'il a dépensé, le bénéficiaire ne voit jamais le prix
 * ligne à ligne. L'API applique déjà cette règle — le champ arrive à `null` sur
 * les cadeaux reçus —, ce composant ne fait que ne pas la contredire.
 */
import { onMounted, ref } from 'vue'
import { useCadeaux, formaterMontant, type MonCadeau } from '~/composables/useCadeaux'

const { listerMesCadeaux } = useCadeaux()

type Sens = 'recus' | 'offerts'

const sens = ref<Sens>('recus')
const cadeaux = ref<MonCadeau[]>([])
const total = ref(0)
const page = ref(1)
const chargement = ref(true)
const TAILLE = 10

const LIBELLE_FAMILLE: Record<string, string> = {
  codimoi: 'Codi-moi',
  factcheck: 'Vérification de faits',
  biblio_humaine: 'Bibliothèque humaine',
  video: 'Vidéo Vidafrica',
  fiche_pays: 'Fiche territoire',
  profil: 'Profil',
  chaine_tv: 'Chaîne TV',
  station_radio: 'Station radio',
  emission_tele: 'Programme TV',
  emission_radio: 'Programme radio',
  episode_tele: 'Épisode TV',
  episode_radio: 'Épisode radio',
  personnalite_connue: 'Personnalité',
  recette_culinaire: 'Recette culinaire',
}

const charger = async (p = 1) => {
  chargement.value = true
  try {
    const res = await listerMesCadeaux(sens.value, p, TAILLE)
    cadeaux.value = res?.elements ?? []
    total.value = res?.total ?? 0
    page.value = res?.page ?? p
  } catch {
    cadeaux.value = []
    total.value = 0
  } finally {
    chargement.value = false
  }
}

const changerSens = (s: Sens) => {
  if (sens.value === s) return
  sens.value = s
  charger(1)
}

onMounted(() => charger(1))

const formaterDate = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' })
</script>

<template>
  <section class="space-y-3">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <h2 class="font-oswald text-xl font-bold text-gray-900">Cadeaux</h2>
      <div class="inline-flex rounded-lg border border-gray-200 p-0.5 text-xs">
        <button
          type="button"
          class="cursor-pointer rounded-md px-3 py-1.5 font-medium transition-colors"
          :class="sens === 'recus' ? 'bg-custom-green text-white' : 'text-gray-600 hover:bg-gray-100'"
          @click="changerSens('recus')"
        >
          Reçus
        </button>
        <button
          type="button"
          class="cursor-pointer rounded-md px-3 py-1.5 font-medium transition-colors"
          :class="sens === 'offerts' ? 'bg-custom-green text-white' : 'text-gray-600 hover:bg-gray-100'"
          @click="changerSens('offerts')"
        >
          Offerts
        </button>
      </div>
    </header>

    <div class="overflow-hidden rounded-2xl border border-gray-100 bg-white">
      <p v-if="chargement" class="px-5 py-8 text-center text-sm text-gray-400">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" />
        Chargement…
      </p>

      <p v-else-if="cadeaux.length === 0" class="px-5 py-8 text-center text-sm text-gray-400">
        {{ sens === 'recus'
          ? 'Vous n\'avez pas encore reçu de cadeau.'
          : 'Vous n\'avez pas encore offert de cadeau.' }}
      </p>

      <ul v-else class="divide-y divide-gray-50">
        <li
          v-for="c in cadeaux"
          :key="c.id"
          class="flex items-start justify-between gap-3 px-5 py-3.5"
        >
          <div class="flex min-w-0 items-start gap-3">
            <font-awesome-icon
              :icon="`fa-solid fa-${c.cadeau.icone || 'gift'}`"
              class="mt-0.5 shrink-0 text-lg text-custom-chocolat"
            />
            <div class="min-w-0">
              <p class="truncate text-sm font-medium text-gray-800">
                {{ c.cadeau.libelle }}
                <span class="font-normal text-gray-500">
                  {{ sens === 'recus' ? 'de' : 'à' }}
                  <NuxtLink
                    :to="`/profil/${c.contrepartie.id}`"
                    class="transition hover:text-custom-chocolat"
                  >{{ c.contrepartie.nom_affiche }}</NuxtLink>
                </span>
              </p>
              <div class="mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-1 text-xs">
                <span class="text-gray-400">{{ formaterDate(c.created_at) }}</span>
                <span class="rounded-full bg-gray-100 px-2 py-0.5 text-gray-600">
                  {{ c.titre_cible || LIBELLE_FAMILLE[c.type_objet] || c.type_objet }}
                </span>
                <span
                  v-if="c.mode === 'points'"
                  class="rounded-full bg-sky-50 px-2 py-0.5 text-sky-700"
                >cadeau en points</span>
                <!-- Le membre doit pouvoir distinguer, ligne à ligne, ce que la
                     purge de fin de phase de test lui retirera. -->
                <span
                  v-if="c.simule"
                  class="rounded-full bg-amber-50 px-2 py-0.5 text-amber-700"
                  title="Paiement simulé — sera retiré à la mise en service du paiement réel"
                >simulé</span>
              </div>
              <em v-if="c.message" class="mt-1 block truncate text-xs text-gray-500">
                « {{ c.message }} »
              </em>
            </div>
          </div>

          <div class="shrink-0 pl-2 text-right">
            <span class="font-oswald text-base font-bold text-custom-green">+{{ c.points }}</span>
            <span
              v-if="c.montant !== null"
              class="block text-[11px] text-gray-400"
            >{{ formaterMontant(c.montant) }}</span>
          </div>
        </li>
      </ul>

      <div
        v-if="total > TAILLE"
        class="flex items-center justify-between border-t border-gray-100 px-5 py-3"
      >
        <button
          type="button"
          class="cursor-pointer text-sm text-gray-500 transition hover:text-gray-900 disabled:opacity-40"
          :disabled="page <= 1"
          @click="charger(page - 1)"
        >
          <font-awesome-icon icon="fa-solid fa-chevron-left" /> Précédent
        </button>
        <span class="text-xs text-gray-400">
          Page {{ page }} sur {{ Math.max(1, Math.ceil(total / TAILLE)) }}
        </span>
        <button
          type="button"
          class="cursor-pointer text-sm text-gray-500 transition hover:text-gray-900 disabled:opacity-40"
          :disabled="page >= Math.ceil(total / TAILLE)"
          @click="charger(page + 1)"
        >
          Suivant <font-awesome-icon icon="fa-solid fa-chevron-right" />
        </button>
      </div>
    </div>
  </section>
</template>
