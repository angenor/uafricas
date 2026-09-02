<script setup lang="ts">
import { radioCategories } from '~/mocks/radios'
import { useStationsRadio } from '~/composables/useStationsRadio'

/**
 * Africans Radio : porté sur le gabarit de la refonte.
 *
 * Le bandeau « 150+ stations · 54 territoires · 24/7 · HD » est SUPPRIMÉ.
 * Ces quatre valeurs étaient écrites en dur dans `mocks/radios.ts` ; aucune
 * requête ne les a jamais vérifiées, et l'endpoint qui fait foi
 * (`/api/stations-radio/sections`) renvoie un tout autre chiffre.
 *
 * Chaque carte porte désormais le décompte RÉEL de sa famille, lu à l'ouverture
 * de la page. `par_page: 1` : seul le total nous intéresse, pas les stations, 
 * inutile de rapatrier la liste entière pour afficher un nombre.
 *
 * Partent aussi les trois encarts « Streaming en direct / Diversité musicale /
 * 24h-24 7j-7 » : deux redisaient les chiffres qu'on retire, le troisième
 * annonçait un catalogue (« Afrobeats, Mbalax, Rumba, Highlife ») que rien dans
 * les données ne garantit.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Africans Radio : Radios africaines | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les meilleures stations de radio africaines. Écoutez en direct les radios africaines internationales et nationales.',
    }],
})

/** Les deux autres applications de l'univers Africamood. */
const AUTRES_MEDIAS = [
  { libelle: 'Télévision africaine', to: '/medias/tele', icone: 'fa-solid fa-tv' },
  { libelle: 'Vidafrica', to: '/vidafrica', icone: 'fa-solid fa-video' }]

const presentationOuverte = ref(false)

const { listerSections } = useStationsRadio()

/** Décompte réel par famille, indexé sur la cible de la carte. */
const comptes = ref<Record<string, number | null>>({
  '/medias/radio/africans': null,
  '/medias/radio/nationales': null,
})

const totalStations = computed(() => {
  const valeurs = Object.values(comptes.value)
  if (valeurs.some(v => v === null)) return null
  return valeurs.reduce<number>((n, v) => n + (v ?? 0), 0)
})

const libelleCompte = (n: number | null) => {
  if (n === null) return '…'
  return `${n} station${n > 1 ? 's' : ''}`
}

onMounted(async () => {
  const [africans, territoire] = await Promise.all([
    listerSections({ origine: 'africans', par_page: 1 }).catch(() => null),
    listerSections({ origine: 'territoire', par_page: 1 }).catch(() => null)])
  comptes.value = {
    '/medias/radio/africans': africans?.total ?? 0,
    '/medias/radio/nationales': territoire?.total ?? 0,
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Africans Radio"
        sous-titre="Écoutez l'Afrique en direct"
        image="/images/banners/radio-home.jpg"
        aide="C'est quoi Africans Radio ?"
        @aide="presentationOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africamood', vers: '/medias' }, { libelle: 'Radios' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">Deux familles de stations</p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <div class="grid gap-5 sm:grid-cols-2">
        <NuxtLink
          v-for="category in radioCategories"
          :key="category.id"
          :to="category.link"
          class="group relative block h-64 overflow-hidden rounded-[10px] border border-af-bordure transition hover:border-af-chocolat"
        >
          <img
            :src="category.image"
            alt=""
            class="absolute inset-0 size-full object-cover transition-transform duration-700 group-hover:scale-105"
            loading="lazy"
          />
          <div class="absolute inset-0 bg-linear-to-t from-black/85 via-black/35 to-transparent" />

          <span
            class="absolute top-4 right-4 rounded px-3 py-1 text-[12px]/[1.4] font-bold text-white"
            :class="category.badgeColor === 'green' ? 'bg-af-vert' : 'bg-af-chocolat'"
          >
            {{ category.badge }}
          </span>

          <div class="relative flex h-full flex-col justify-end gap-2 p-6 text-white">
            <h2 class="text-[24px]/[1.3] font-bold">{{ category.title }}</h2>
            <p class="line-clamp-2 text-[14px]/[1.5] text-white/90">{{ category.description }}</p>
            <span class="mt-1 flex items-center gap-2 text-[14px]/[1.4] font-bold">
              Explorer, {{ libelleCompte(comptes[category.link] ?? null) }}
              <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition-transform group-hover:translate-x-1" />
            </span>
          </div>
        </NuxtLink>
      </div>

      <!-- Texte d'accueil : il présente le module, il n'avance aucun chiffre. -->
      <section class="rounded-[10px] border border-af-bordure bg-white p-8 text-center">
        <h2 class="text-[24px]/[1.3] font-bold text-af-encre">Votre passerelle vers les sons africains</h2>
        <p class="mx-auto mt-3 max-w-2xl text-[14px]/[1.6] text-af-corps">
          Plongez dans la diversité musicale et culturelle de l'Afrique à travers nos stations de radio.
          Des rythmes traditionnels aux hits afrobeats les plus récents, découvrez la richesse sonore du continent.
        </p>
      </section>
    </div>

    <template #rail>
      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div
            v-for="(category, i) in radioCategories"
            :key="category.id"
            class="flex items-baseline justify-between gap-4 py-3"
            :class="i > 0 && 'border-t border-af-bordure'"
          >
            <dt class="text-[14px]/[1.4] font-bold">{{ category.title }}</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">
              {{ comptes[category.link] ?? '…' }}
            </dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure pt-3">
            <dt class="text-[14px]/[1.4] font-bold">Total</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ totalStations ?? '…' }}</dd>
          </div>
        </dl>
      </AfricansPanneau>

      <AfricansPanneau titre="Aussi dans Africamood" icone="fa-solid fa-photo-film">
        <ul class="flex flex-col gap-1">
          <li v-for="lien in AUTRES_MEDIAS" :key="lien.to">
            <NuxtLink
              :to="lien.to"
              class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] font-bold text-af-corps transition hover:bg-af-chocolat/[0.07] hover:text-af-chocolat"
            >
              <font-awesome-icon :icon="lien.icone" class="size-5 shrink-0" />
              {{ lien.libelle }}
            </NuxtLink>
          </li>
        </ul>
      </AfricansPanneau>
    </template>

    <MediaRadioDecouverteModale v-model="presentationOuverte" />
  </NuxtLayout>
</template>
