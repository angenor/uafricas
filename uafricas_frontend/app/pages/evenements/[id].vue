<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="evenement?.titre ?? 'Événement'"
        :sous-titre="evenement ? [evenement.ville, evenement.pays].filter(Boolean).join(', ') || undefined : undefined"
        :image="evenement?.couverture_url ? resoudreUrlImage(evenement.couverture_url) : null"
      >
        <template v-if="evenement" #action>
          <div class="flex items-center gap-2">
            <AfricansEtiquette v-if="evenement.thematique">{{ evenement.thematique }}</AfricansEtiquette>
            <span class="rounded-lg bg-af-vert px-4 py-2 text-[14px]/[1.4] font-bold text-white">
              {{ labelStatut }}
            </span>
          </div>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Africalive', vers: '/evenements' },
          { libelle: 'Événements', vers: '/evenements/liste' },
          { libelle: evenement?.titre ?? 'Événement' }]"
      />
    </template>

    <div v-if="chargement && !evenement" class="flex flex-col gap-5">
      <div v-for="n in 3" :key="n" class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="!evenement" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-calendar-xmark" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Événement introuvable</p>
      <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreur || "Il a peut-être été annulé ou retiré." }}</p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" vers="/evenements/liste">
        Retour aux événements
      </AfricansBouton>
    </div>

    <div v-else class="flex flex-col gap-5">
      <!-- Le direct passe AVANT tout le reste : quand il est ouvert, c'est la
           seule chose que le visiteur cherche. -->
      <div
        v-if="etatDirect && etatDirect.statut_direct === 'en_direct'"
        class="flex flex-wrap items-center gap-4 rounded-[10px] border border-af-live/40 bg-af-live/5 p-5"
      >
        <span class="flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-live">
          <span class="size-2.5 animate-pulse rounded-full bg-af-live" />
          En direct
        </span>
        <span class="text-[14px]/[1.4] text-af-corps">
          {{ etatDirect.nombre_participants }} participant(s) connecté(s)
        </span>
        <AfricansBouton
          v-if="etatDirect.peut_rejoindre"
          class="ml-auto"
          icone="fa-solid fa-video"
          @click="rejoindreLeDirect"
        >
          {{ etatDirect.est_organisateur ? 'Rejoindre mon direct' : 'Rejoindre le direct' }}
        </AfricansBouton>
        <AfricansBouton
          v-else-if="!isAuthenticated"
          class="ml-auto"
          icone="fa-solid fa-right-to-bracket"
          :vers="`/login?redirect=/evenements/${evenementId}`"
        >
          Connectez-vous pour rejoindre
        </AfricansBouton>
        <span v-else-if="!etatDirect.est_inscrit" class="ml-auto text-[14px]/[1.4] font-bold text-af-live">
          Inscrivez-vous pour rejoindre le direct
        </span>
        <span v-else class="ml-auto text-[14px]/[1.4] font-bold text-af-live">
          Capacité atteinte, réessayez plus tard
        </span>
      </div>

      <div
        v-else-if="etatDirect && etatDirect.statut_direct === 'en_attente' && etatDirect.peut_ouvrir"
        class="flex flex-wrap items-center gap-4 rounded-[10px] border border-af-chocolat/30 bg-af-chocolat/5 p-5"
      >
        <span class="text-[14px]/[1.4] text-af-corps">Le direct n'est pas encore ouvert.</span>
        <AfricansBouton class="ml-auto" icone="fa-solid fa-video" @click="rejoindreLeDirect">
          Ouvrir le direct
        </AfricansBouton>
      </div>

      <p
        v-else-if="etatDirect && etatDirect.statut_direct === 'termine'"
        class="rounded-[10px] border border-af-bordure bg-white p-5 text-[14px]/[1.4] text-af-corps"
      >
        Le direct de cet événement est terminé.
      </p>

      <AfricansAccordeon titre="Informations pratiques" icone="fa-solid fa-circle-info" fond="blanc" par-defaut-ouvert>
        <dl class="grid gap-5 sm:grid-cols-2">
          <div class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-calendar-days" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[12px]/[1.4] text-af-atone">Date</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">
                {{ formatDateShort(evenement.date_heure_debut) }}
                <template v-if="evenement.date_heure_fin">, {{ formatDateShort(evenement.date_heure_fin) }}</template>
              </dd>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-clock" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[12px]/[1.4] text-af-atone">Horaire</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">
                {{ getHeure(evenement.date_heure_debut) }}
                <template v-if="evenement.date_heure_fin">, {{ getHeure(evenement.date_heure_fin) }}</template>
              </dd>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <font-awesome-icon :icon="formatIcon" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[12px]/[1.4] text-af-atone">Format</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ evenement.type }}</dd>
            </div>
          </div>

          <div v-if="evenement.ville || evenement.pays" class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-location-dot" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[12px]/[1.4] text-af-atone">Lieu</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">
                {{ [evenement.ville, evenement.pays].filter(Boolean).join(', ') }}
              </dd>
            </div>
          </div>

          <div v-if="evenement.adresse" class="flex items-start gap-3 sm:col-span-2">
            <font-awesome-icon icon="fa-solid fa-map-pin" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[12px]/[1.4] text-af-atone">Adresse</dt>
              <dd class="text-[14px]/[1.4] text-af-encre">{{ evenement.adresse }}</dd>
            </div>
          </div>

          <div v-if="evenement.lien_en_ligne" class="flex items-start gap-3 sm:col-span-2">
            <font-awesome-icon icon="fa-solid fa-link" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[12px]/[1.4] text-af-atone">Lien de participation</dt>
              <dd>
                <a
                  :href="evenement.lien_en_ligne"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
                >
                  {{ evenement.lien_en_ligne }}
                </a>
              </dd>
            </div>
          </div>
        </dl>
      </AfricansAccordeon>

      <AfricansAccordeon v-if="evenement.description" titre="Description" icone="fa-solid fa-align-left" par-defaut-ouvert>
        <!-- eslint-disable-next-line vue/no-v-html -->
        <div class="text-[14px]/[1.6] text-af-corps" v-html="sanitiserHtml(evenement.description)" />
      </AfricansAccordeon>

      <AfricansAccordeon v-if="estTermine && enregistrementUrl" titre="Rediffusion" icone="fa-solid fa-play" par-defaut-ouvert>
        <div v-if="enregistrementEmbed" class="aspect-video w-full overflow-hidden rounded-[10px]">
          <iframe
            :src="enregistrementEmbed"
            class="size-full"
            title="Rediffusion de l'événement"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
            allowfullscreen
          />
        </div>
        <!-- Repli : l'URL n'est pas une vidéo YouTube reconnue. -->
        <a
          v-else
          :href="enregistrementUrl"
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
        >
          <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" />
          Voir la rediffusion
        </a>
      </AfricansAccordeon>

      <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-left" vers="/evenements/liste" class="self-start">
        Retour aux événements
      </AfricansBouton>
    </div>

    <template #rail>
      <template v-if="evenement">
        <AfricansPanneau titre="Inscription" icone="fa-solid fa-user-plus">
          <div class="flex flex-col gap-3">
            <p class="text-[14px]/[1.4] text-af-corps">
              <span class="text-[20px]/[1.4] font-bold text-af-chocolat">{{ evenement.nombre_inscrits }}</span>
              inscrit{{ evenement.nombre_inscrits > 1 ? 's' : '' }}
              <template v-if="evenement.nombre_places">
                sur {{ evenement.nombre_places }} place{{ evenement.nombre_places > 1 ? 's' : '' }}
              </template>
            </p>

            <template v-if="isAuthenticated">
              <p v-if="isInscrit" class="flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-vert">
                <font-awesome-icon icon="fa-solid fa-circle-check" />
                Inscription confirmée
              </p>
              <AfricansBouton v-else pleine-largeur icone="fa-solid fa-user-plus" @click="handleInscription">
                S'inscrire à cet événement
              </AfricansBouton>
            </template>

            <template v-else>
              <p class="text-[14px]/[1.4] text-af-corps">Connectez-vous pour vous inscrire.</p>
              <AfricansBouton pleine-largeur icone="fa-solid fa-right-to-bracket" :vers="`/login?redirect=/evenements/${evenementId}`">
                Se connecter
              </AfricansBouton>
            </template>
          </div>
        </AfricansPanneau>

        <AfricansPanneau v-if="evenement.user" titre="Organisateur" icone="fa-solid fa-user-tie">
          <div class="flex flex-col gap-4">
            <div class="flex items-center gap-3">
              <img
                v-if="urlMedia(evenement.user.photo_url)"
                :src="urlMedia(evenement.user.photo_url)!"
                :alt="`${evenement.user.prenom} ${evenement.user.nom}`"
                class="size-12 shrink-0 rounded-full object-cover"
              />
              <span v-else class="grid size-12 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-[17px]/[1.4] font-bold text-af-chocolat">
                {{ evenement.user.prenom?.charAt(0) }}{{ evenement.user.nom?.charAt(0) }}
              </span>
              <div class="min-w-0">
                <p class="text-[14px]/[1.4] font-bold text-af-encre">
                  {{ evenement.user.prenom }} {{ evenement.user.nom }}
                </p>
                <p
                  v-if="evenement.type_organisateur === 'organisation' && evenement.contact_nom"
                  class="mt-1 flex items-center gap-1.5 text-[12px]/[1.4] font-bold text-af-chocolat"
                >
                  <font-awesome-icon icon="fa-solid fa-building" class="shrink-0" />
                  <span class="min-w-0 truncate">Au nom de {{ evenement.contact_nom }}</span>
                </p>
                <p v-else class="mt-1 flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
                  <font-awesome-icon icon="fa-solid fa-user" class="shrink-0" />
                  Publié en nom propre
                </p>
              </div>
            </div>

            <ul v-if="aContact" class="flex flex-col gap-2 border-t border-af-bordure pt-4">
              <li v-if="evenement.contact_email" class="flex items-center gap-3 text-[14px]/[1.4]">
                <font-awesome-icon icon="fa-solid fa-envelope" class="w-4 shrink-0 text-af-atone" />
                <a :href="`mailto:${evenement.contact_email}`" class="min-w-0 truncate text-af-corps transition hover:text-af-chocolat">
                  {{ evenement.contact_email }}
                </a>
              </li>
              <li v-if="evenement.contact_telephone" class="flex items-center gap-3 text-[14px]/[1.4]">
                <font-awesome-icon icon="fa-solid fa-phone" class="w-4 shrink-0 text-af-atone" />
                <a :href="`tel:${evenement.contact_telephone}`" class="text-af-corps transition hover:text-af-chocolat">
                  {{ evenement.contact_telephone }}
                </a>
              </li>
              <li v-if="evenement.contact_site_web" class="flex items-center gap-3 text-[14px]/[1.4]">
                <font-awesome-icon icon="fa-solid fa-globe" class="w-4 shrink-0 text-af-atone" />
                <a
                  :href="evenement.contact_site_web"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="min-w-0 truncate text-af-corps transition hover:text-af-chocolat"
                >
                  {{ evenement.contact_site_web }}
                </a>
              </li>
            </ul>
          </div>
        </AfricansPanneau>

        <AfricansPanneau titre="Partager" icone="fa-solid fa-share-nodes">
          <EvenementsEvenementPartage
            :path="`/evenements/${evenementId}`"
            :evenement-id="evenementId"
            :titre="evenement.titre"
          />
        </AfricansPanneau>
      </template>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">

definePageMeta({ layout: false })
import { useEvenements, formatDateShort, getHeure, type EvenementDetailAPI, type EtatDirect } from '~/composables/useEvenements'
import { youtubeEmbedUrl } from '~/utils/media'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const evenementId = route.params.id as string
const userStore = useUserStore()

const { obtenirEvenement, inscrireEvenement, obtenirEtatDirect, signalStream, chargement, erreur } = useEvenements()

// Chargement SSR : indispensable pour que les balises Open Graph soient présentes
// dans le HTML lu par les robots des réseaux sociaux (aperçu lors du partage).
const { data: evenementCharge } = await useAsyncData(
  `evenement-${evenementId}`,
  () => obtenirEvenement(evenementId),
)
const evenement = ref<EvenementDetailAPI | null>(evenementCharge.value)
const isInscrit = ref(evenementCharge.value?.est_inscrit ?? false)
const isAuthenticated = computed(() => !!userStore.accessToken)

// État du direct (feature 001-evenements-streaming), rafraîchi via SSE.
const etatDirect = ref<EtatDirect | null>(null)
const chargerEtatDirect = async (): Promise<void> => {
  etatDirect.value = await obtenirEtatDirect(evenementId)
}
const rejoindreLeDirect = (): Promise<unknown> => navigateTo(`/evenements/${evenementId}/direct`)

// Rafraîchit l'état du direct quand un évènement SSE concerne cet événement.
watch(signalStream, (s) => {
  if (s && s.evenement_id === evenementId) void chargerEtatDirect()
})

onMounted(async () => {
  // Rechargement authentifié côté client : le rendu SSR est anonyme, on récupère
  // ici l'état d'inscription réel de l'utilisateur connecté.
  const data = await obtenirEvenement(evenementId)
  if (data) {
    evenement.value = data
    isInscrit.value = data.est_inscrit
  }
  await chargerEtatDirect()
})

const breadcrumbs = computed(() => [
  { label: 'Événements', to: '/evenements' },
  { label: evenement.value?.titre || 'Détail', to: undefined }
])

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const requete = useRequestURL()
const origineSite = `${requete.protocol}//${requete.host}`
const urlCanonique = `${origineSite}/evenements/${evenementId}`

const resoudreUrlImage = (url: string | null | undefined): string => {
  if (!url) return ''
  if (url.startsWith('http://') || url.startsWith('https://')) return url
  return `${apiBase}${url}`
}

const imageOg = computed(() => resoudreUrlImage(evenement.value?.couverture_url))
const descriptionOg = computed(() => {
  const e = evenement.value
  if (!e) return 'Événements & ateliers panafricains | UAfricas'
  const lieu = [e.ville, e.pays].filter(Boolean).join(', ')
  const brut = (e.description || '').replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim()
  const resume = brut.length > 160 ? `${brut.slice(0, 157)}…` : brut
  return resume || `Rejoignez « ${e.titre} »${lieu ? ` à ${lieu}` : ''} sur UAfricas.`
})

useHead(() => {
  const titre = evenement.value ? `${evenement.value.titre} | UAfricas` : 'Événement | UAfricas'
  const img = imageOg.value
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      // Open Graph
      { property: 'og:type', content: 'article' },
      { property: 'og:title', content: titre },
      { property: 'og:description', content: descriptionOg.value },
      { property: 'og:url', content: urlCanonique },
      { property: 'og:site_name', content: 'UAfricas' }, ...(img ? [{ property: 'og:image', content: img }] : []),
      // Twitter Card
      { name: 'twitter:card', content: img ? 'summary_large_image' : 'summary' },
      { name: 'twitter:title', content: titre },
      { name: 'twitter:description', content: descriptionOg.value }, ...(img ? [{ name: 'twitter:image', content: img }] : [])],
    link: [{ rel: 'canonical', href: urlCanonique }],
  }
})

// --- Computed pour les badges ---

const badgeFormatClasses = computed(() => {
  const type = evenement.value?.type || ''
  if (type.includes('ligne')) return 'bg-blue-600'
  if (type.includes('Hybride') || type.includes('hybride')) return 'bg-purple-600'
  return 'bg-custom-chocolat'
})

const formatIcon = computed(() => {
  const type = evenement.value?.type || ''
  if (type.includes('ligne')) return 'fa-solid fa-video'
  if (type.includes('Hybride') || type.includes('hybride')) return 'fa-solid fa-shuffle'
  return 'fa-solid fa-map-marker-alt'
})

const badgeStatutClasses = computed(() => {
  switch (evenement.value?.statut) {
    case 'a_venir': return 'bg-green-100 text-green-700'
    case 'en_cours': return 'bg-blue-100 text-blue-700'
    case 'termine': return 'bg-gray-200 text-gray-600'
    case 'annule': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-200 text-gray-600'
  }
})

const labelStatut = computed(() => {
  switch (evenement.value?.statut) {
    case 'a_venir': return 'À venir'
    case 'en_cours': return 'En cours'
    case 'termine': return 'Terminé'
    case 'annule': return 'Annulé'
    default: return evenement.value?.statut || ''
  }
})

const aContact = computed(() => {
  const e = evenement.value
  return !!(e?.contact_email || e?.contact_telephone || e?.contact_site_web)
})

// Enregistrement vidéo : affiché une fois l'événement terminé.
const estTermine = computed(() => evenement.value?.statut === 'termine')
const enregistrementUrl = computed(() => evenement.value?.enregistrement_url || null)
const enregistrementEmbed = computed(() => youtubeEmbedUrl(enregistrementUrl.value))

const handleInscription = async () => {
  const success = await inscrireEvenement(evenementId)
  if (success) {
    isInscrit.value = true
  }
}
</script>
