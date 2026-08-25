<script setup lang="ts">
import type { BiblioHumaineAPI } from '~/composables/useBibliothequeHumaine'
import type { ExpertAPI } from '~/composables/useExperts'
import type { MembreAPI } from '~/composables/useMembres'
import type { EtatRelation, MembreLightAPI } from '~/composables/useAmis'
import type { BadgeObtenu } from '~/composables/useEngagement'
import { useUserStore } from '~/stores/user'

definePageMeta({ layout: false })

const route = useRoute()
const id = route.params.id as string

const { obtenirBiblio } = useBibliothequeHumaine()
const { obtenirExpert } = useExperts()
const { obtenirMembre, partagerProfil, signalerProfil } = useMembres()
const { obtenirEtatRelation } = useAmis()
const { demanderOuverture } = useMessagerie()
const { obtenirBadgesPublics } = useEngagement()
const userStore = useUserStore()

// Badges obtenus, affichés publiquement à côté du badge de statut (FR-014).
// Aucun solde ni journal n'est exposé ici : le détail chiffré reste privé.
const badgesPublics = ref<BadgeObtenu[]>([])

/** Jetons de couleur (base) → classes Tailwind, alignés sur `EngagementBadgeSucces`. */
const CLASSES_BADGE: Record<string, string> = {
  green: 'bg-custom-green/10 text-custom-green ring-custom-green/30',
  chocolat: 'bg-custom-chocolat/10 text-custom-chocolat ring-custom-chocolat/30',
  amber: 'bg-amber-50 text-amber-700 ring-amber-200',
  sky: 'bg-sky-50 text-sky-700 ring-sky-200',
  violet: 'bg-violet-50 text-violet-700 ring-violet-200',
  rose: 'bg-rose-50 text-rose-600 ring-rose-200',
  slate: 'bg-slate-100 text-slate-700 ring-slate-300',
  gray: 'bg-gray-100 text-gray-600 ring-gray-200',
}

const classesBadge = (couleur: string | null) =>
  CLASSES_BADGE[couleur || 'gray'] || CLASSES_BADGE.gray

// État de la relation avec ce membre (FR-016)
const etatRelation = ref<EtatRelation>('aucune')
const estMoi = computed(() => userStore.user?.id === id)
const peutAfficherAmitie = computed(() => userStore.isAuthenticated && !estMoi.value)
// La messagerie n'est ouverte qu'entre amis (FR-022)
const peutEnvoyerMessage = computed(() => peutAfficherAmitie.value && etatRelation.value === 'amis')
// Tout membre connecté peut noter un expert, hors son propre profil
const peutNoter = computed(() => userStore.isAuthenticated && !estMoi.value)

/** Met à jour la note moyenne / sa note après notation. */
const surNote = (payload: { rating: number, nombreNotes: number, maNote: number }) => {
  if (!expert.value) return
  expert.value.expertiseInfo.rating = payload.rating
  expert.value.expertiseInfo.nombreNotes = payload.nombreNotes
  expert.value.expertiseInfo.maNote = payload.maNote
}

/** MembreLight de ce profil (pour la messagerie et la proposition de RDV). */
const membreLight = computed<MembreLightAPI | null>(() => {
  if (!membre.value) return null
  return {
    id: membre.value.id,
    nom: membre.value.nom,
    prenom: membre.value.prenom,
    slug: membre.value.slug,
    photoUrl: membre.value.photoUrl,
    fonction: membre.value.fonction,
    pays: membre.value.pays,
  }
})

/** Ouvre la fenêtre flottante de messagerie sur la conversation de ce profil. */
const ouvrirMessagerie = () => {
  if (membreLight.value) demanderOuverture(membreLight.value)
}

// Proposition de rendez-vous en visioconférence (entre amis, FR-001).
const afficherModalRdv = ref(false)

// Partage & signalement du profil
const afficherModalPartage = ref(false)
const afficherModalSignalement = ref(false)
const profilSignale = ref(false)
const modalPartageRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)
const modalSignalementRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: (m: string) => void } | null>(null)

/** Partage le profil sur le mur communautaire (/publications). */
const soumettrePartage = async (legende: string) => {
  modalPartageRef.value?.setLoading(true)
  try {
    const res = await partagerProfil(id, legende || undefined)
    if (res) modalPartageRef.value?.setSuccess()
    else modalPartageRef.value?.setError('Erreur lors du partage. Veuillez réessayer.')
  }
  catch (e: any) {
    modalPartageRef.value?.setError(e?.message || 'Erreur lors du partage.')
  }
}

/** Signale le profil (faux profil / arnaque). */
const soumettreSignalement = async (payload: { motif: string, description: string }) => {
  modalSignalementRef.value?.setLoading(true)
  try {
    const etat = await signalerProfil(id, payload.motif, payload.description || undefined)
    if (etat) {
      profilSignale.value = true
      const message = etat.deja_signale
        ? 'Vous aviez déjà signalé ce profil.'
        : 'Merci, votre signalement a été pris en compte.'
      modalSignalementRef.value?.setSuccess(message)
    }
    else {
      modalSignalementRef.value?.setError('Erreur lors du signalement. Veuillez réessayer.')
    }
  }
  catch (e: any) {
    modalSignalementRef.value?.setError(e?.message || 'Erreur lors du signalement.')
  }
}

const chargement = ref(true)
const membre = ref<MembreAPI | null>(null)
const biblio = ref<BiblioHumaineAPI | null>(null)
const expert = ref<ExpertAPI | null>(null)

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const estBiblio = computed(() => biblio.value !== null)
const estExpert = computed(() => expert.value !== null && expert.value.expertiseInfo.statut === 'valide')

const profil = computed(() => {
  // Base : profil membre (present pour tout compte actif), enrichi par biblio/expert
  if (membre.value) {
    return {
      id: membre.value.id,
      nom: membre.value.nom,
      prenom: membre.value.prenom,
      photoUrl: membre.value.photoUrl,
      fonction: membre.value.fonction || biblio.value?.fonction || expert.value?.expertiseInfo.domaine || null,
      pays: membre.value.pays || biblio.value?.pays || expert.value?.pays || null,
      ville: membre.value.ville || biblio.value?.ville || expert.value?.ville || null,
      dateInscription: membre.value.dateInscription,
    }
  }
  // Fallback si l'endpoint membre echoue mais biblio/expert disponible
  if (biblio.value) {
    return {
      id: biblio.value.userId,
      nom: biblio.value.nom,
      prenom: biblio.value.prenom,
      photoUrl: biblio.value.photoUrl,
      fonction: biblio.value.fonction,
      pays: biblio.value.pays,
      ville: biblio.value.ville,
      dateInscription: biblio.value.dateInscription,
    }
  }
  if (expert.value) {
    return {
      id: expert.value.id,
      nom: expert.value.nom,
      prenom: expert.value.prenom,
      photoUrl: expert.value.photoURL,
      fonction: expert.value.expertiseInfo.domaine,
      pays: expert.value.pays,
      ville: expert.value.ville,
      dateInscription: expert.value.dateInscription,
    }
  }
  return null
})

const ongletActif = ref<'apropos' | 'biblio' | 'expert'>('apropos')

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const onglets = computed(() => {
  const items: Array<{ id: 'apropos' | 'biblio' | 'expert', label: string, icon: string }> = [
    { id: 'apropos', label: 'À propos', icon: 'fa-solid fa-user' },
  ]
  if (estBiblio.value) items.push({ id: 'biblio', label: 'Bibliothèque Humaine', icon: 'fa-solid fa-book-open' })
  if (estExpert.value) items.push({ id: 'expert', label: 'Expertise', icon: 'fa-solid fa-briefcase' })
  return items
})

useHead({
  title: computed(() =>
    profil.value
      ? `${profil.value.prenom} ${profil.value.nom} | AfricanS`
      : 'Profil | AfricanS',
  ),
})

const photoComplete = computed(() => {
  const url = profil.value?.photoUrl
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
})

const dateInscriptionFormatee = computed(() => {
  if (!profil.value?.dateInscription) return ''
  return new Date(profil.value.dateInscription).toLocaleDateString('fr-FR', {
    month: 'long',
    year: 'numeric',
  })
})

onMounted(async () => {
  chargement.value = true
  const [m, b, e, bg] = await Promise.all([
    obtenirMembre(id).catch(() => null),
    obtenirBiblio(id).catch(() => null),
    obtenirExpert(id).catch(() => null),
    obtenirBadgesPublics(id).catch(() => []),
  ])
  membre.value = m
  biblio.value = b
  expert.value = e
  badgesPublics.value = bg

  if (estBiblio.value) ongletActif.value = 'biblio'
  else if (estExpert.value) ongletActif.value = 'expert'

  chargement.value = false

  // Charger l'état de relation (membre connecté, hors propre profil)
  if (peutAfficherAmitie.value) {
    const rel = await obtenirEtatRelation(id)
    if (rel) etatRelation.value = rel.etat
  }
})

/**
 * Sur son PROPRE profil, le bloc « Entrer en contact » n'a pas de sens : il
 * proposait un bouton « Envoyer un message » désactivé sous la mention
 * « Connectez-vous pour échanger avec ce membre », adressée à un visiteur
 * déconnecté, elle s'affichait aussi au membre connecté qui regardait sa propre
 * fiche, puisque `peutAfficherAmitie` mêle les deux cas.
 */
const estVisiteurAnonyme = computed(() => !userStore.isAuthenticated)
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="profil ? `${profil.prenom} ${profil.nom}` : 'Profil'"
        :sous-titre="profil?.fonction || undefined"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Communauté', vers: '/profil' },
          { libelle: profil ? `${profil.prenom} ${profil.nom}` : 'Profil' }]"
      >
        <template v-if="estMoi" #action>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-user-pen" vers="/mon-compte/profil">
            Modifier mon profil
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div v-if="chargement" class="flex flex-col gap-6">
      <div class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
      <div class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="!profil" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-user-slash" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Profil introuvable</p>
      <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
        Ce profil n'existe pas ou n'est pas visible publiquement.
      </p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-users" vers="/profil">
        Retour à l'annuaire
      </AfricansBouton>
    </div>

    <div v-else class="flex flex-col gap-6">
      <!-- Identité -->
      <section class="flex flex-col items-center gap-5 rounded-[10px] border border-af-bordure bg-white p-6 text-center sm:flex-row sm:items-start sm:text-left">
        <AfricansAvatar :nom="`${profil.prenom} ${profil.nom}`" :src="photoComplete" :taille="112" />

        <div class="min-w-0 flex-1">
          <div class="flex flex-wrap items-center justify-center gap-2 sm:justify-start">
            <h1 class="text-[24px]/[1.3] font-bold text-af-encre">{{ profil.prenom }} {{ profil.nom }}</h1>
            <EngagementBadgeStatut :utilisateur-id="id" taille="sm" />
          </div>
          <p v-if="profil.fonction" class="mt-1 text-[16px]/[1.4] font-bold text-af-chocolat">
            {{ profil.fonction }}
          </p>

          <div v-if="estBiblio || estExpert" class="mt-3 flex flex-wrap justify-center gap-2 sm:justify-start">
            <AfricansEtiquette v-if="estExpert" ton="vert">Expert</AfricansEtiquette>
            <AfricansEtiquette v-if="estBiblio">Bibliothèque humaine</AfricansEtiquette>
          </div>

          <!--
            Badges obtenus. Volontairement SANS solde, réputation ni journal :
            le détail chiffré de l'engagement reste privé (FR-014), seule la
            distinction est publique.
          -->
          <div v-if="badgesPublics.length > 0" class="mt-3 flex flex-wrap justify-center gap-1.5 sm:justify-start">
            <span
              v-for="b in badgesPublics"
              :key="b.code"
              class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-[12px]/[1.4] font-bold ring-1"
              :class="classesBadge(b.couleur)"
              :title="b.description"
            >
              <font-awesome-icon :icon="`fa-solid fa-${b.icone || 'award'}`" />
              {{ b.libelle }}
            </span>
          </div>

          <div class="mt-3 flex flex-wrap justify-center gap-x-4 gap-y-2 text-[14px]/[1.4] text-af-corps sm:justify-start">
            <span v-if="profil.ville || profil.pays" class="flex items-center gap-1.5">
              <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-chocolat" />
              {{ [profil.ville, profil.pays].filter(Boolean).join(', ') }}
            </span>
            <span v-if="dateInscriptionFormatee" class="flex items-center gap-1.5 text-af-atone">
              <font-awesome-icon icon="fa-solid fa-calendar" />
              Membre depuis {{ dateInscriptionFormatee }}
            </span>
          </div>
        </div>
      </section>

      <!-- Onglets : « À propos » toujours, les deux autres selon les rôles -->
      <section class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
        <AfricansOnglets
          v-if="onglets.length > 1"
          :model-value="ongletActif"
          :onglets="onglets.map(o => ({ valeur: o.id, libelle: o.label }))"
          @update:model-value="(v: string) => ongletActif = v as typeof ongletActif"
        />

        <div class="p-6">
          <!-- À propos -->
          <div v-if="ongletActif === 'apropos'" class="flex flex-col gap-3">
            <h2 class="text-[17px]/[1.4] font-bold">À propos</h2>
            <p
              v-if="membre?.biographie || biblio?.biographie || expert?.expertiseInfo.biographie"
              class="text-[14px]/[1.6] whitespace-pre-line text-af-corps"
            >
              {{ membre?.biographie || biblio?.biographie || expert?.expertiseInfo.biographie }}
            </p>
            <p v-else class="text-[14px]/[1.4] text-af-atone">
              {{ estMoi ? "Vous n'avez pas encore renseigné de biographie." : 'Aucune biographie disponible.' }}
            </p>
            <AfricansBouton
              v-if="estMoi && !membre?.biographie"
              class="self-start"
              variante="secondaire"
              icone="fa-solid fa-pen"
              vers="/mon-compte/profil"
            >
              Compléter mon profil
            </AfricansBouton>
          </div>

          <!-- Bibliothèque humaine -->
          <div v-else-if="ongletActif === 'biblio' && biblio" class="flex flex-col gap-5">
            <h2 class="flex items-center gap-2 text-[17px]/[1.4] font-bold">
              <font-awesome-icon icon="fa-solid fa-book-open" class="text-af-chocolat" />
              Bibliothèque humaine
            </h2>
            <p class="text-[14px]/[1.6] whitespace-pre-line text-af-corps">{{ biblio.biographie }}</p>

            <div v-if="biblio.specialites.length > 0">
              <h3 class="mb-2 text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">
                Domaines d'expertise
              </h3>
              <div class="flex flex-wrap gap-2">
                <AfricansEtiquette v-for="s in biblio.specialites" :key="s">{{ s }}</AfricansEtiquette>
              </div>
            </div>

            <BibliothequeInteractions :biblio="biblio" :peut-interagir="peutNoter" />
          </div>

          <!-- Expertise -->
          <div v-else-if="ongletActif === 'expert' && expert" class="flex flex-col gap-5">
            <h2 class="flex items-center gap-2 text-[17px]/[1.4] font-bold">
              <font-awesome-icon icon="fa-solid fa-briefcase" class="text-af-vert" />
              Profil expert
            </h2>

            <div class="grid gap-4 sm:grid-cols-3">
              <div class="rounded-[10px] bg-af-fond p-4">
                <p class="mb-1 text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">Domaine</p>
                <p class="text-[14px]/[1.4] font-bold">{{ expert.expertiseInfo.domaine }}</p>
              </div>
              <div class="rounded-[10px] bg-af-fond p-4">
                <p class="mb-1 text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">Expérience</p>
                <p class="text-[14px]/[1.4] font-bold">{{ expert.expertiseInfo.nbAnneesExperience }} ans</p>
              </div>
              <ExpertsNotationExpert
                :utilisateur-id="expert.id"
                :rating="expert.expertiseInfo.rating"
                :nombre-notes="expert.expertiseInfo.nombreNotes"
                :ma-note="expert.expertiseInfo.maNote"
                :peut-noter="peutNoter"
                @note="surNote"
              />
            </div>

            <div>
              <h3 class="mb-2 text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">Biographie</h3>
              <p class="text-[14px]/[1.6] whitespace-pre-line text-af-corps">{{ expert.expertiseInfo.biographie }}</p>
            </div>

            <div v-if="expert.situationProfessionnelle.length > 0">
              <h3 class="mb-2 text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">
                Situation professionnelle
              </h3>
              <div class="flex flex-wrap gap-2">
                <AfricansEtiquette v-for="s in expert.situationProfessionnelle" :key="s">{{ s }}</AfricansEtiquette>
              </div>
            </div>

            <div v-if="expert.expertiseInfo.portfolio">
              <h3 class="mb-2 text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">Portfolio</h3>
              <a
                :href="expert.expertiseInfo.portfolio"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-1.5 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
              >
                <font-awesome-icon icon="fa-solid fa-link" />
                {{ expert.expertiseInfo.portfolio }}
              </a>
            </div>
          </div>
        </div>
      </section>
    </div>

    <template #rail>
      <template v-if="profil">
      <!-- Sur son propre profil, ce panneau ne propose rien : il est remplacé
           par le raccourci d'édition, seule action qui ait un sens ici. -->
      <AfricansPanneau v-if="estMoi" titre="Mon profil" icone="fa-solid fa-user">
        <p class="mb-4 text-[14px]/[1.4] text-af-corps">
          Voici votre fiche telle que les autres membres la voient.
        </p>
        <AfricansBouton pleine-largeur icone="fa-solid fa-user-pen" vers="/mon-compte/profil">
          Modifier mon profil
        </AfricansBouton>
      </AfricansPanneau>

      <AfricansPanneau v-else titre="Entrer en contact" icone="fa-solid fa-comments">
        <div class="flex flex-col gap-3">
          <div v-if="peutAfficherAmitie" class="flex justify-center">
            <SocialBoutonAmitie :utilisateur-id="id" :etat="etatRelation" @update="(e: EtatRelation) => etatRelation = e" />
          </div>

          <AfricansBouton
            pleine-largeur
            icone="fa-solid fa-envelope"
            :desactive="!peutEnvoyerMessage"
            @click="ouvrirMessagerie"
          >
            Envoyer un message
          </AfricansBouton>

          <AfricansBouton
            v-if="peutEnvoyerMessage"
            pleine-largeur
            variante="secondaire"
            icone="fa-solid fa-video"
            @click="afficherModalRdv = true"
          >
            Proposer un rendez-vous
          </AfricansBouton>

          <!-- Les deux mentions répondent à deux situations distinctes, que
               l'ancienne condition confondait. -->
          <p v-if="estVisiteurAnonyme" class="text-[12px]/[1.4] text-af-atone">
            Connectez-vous pour échanger avec ce membre.
          </p>
          <p v-else-if="!peutEnvoyerMessage" class="text-[12px]/[1.4] text-af-atone">
            Vous devez être ami(e)s pour envoyer un message.
          </p>
        </div>
      </AfricansPanneau>

      <AfricansPanneau v-if="peutNoter" titre="Actions" icone="fa-solid fa-ellipsis">
        <div class="flex flex-col gap-3">
          <AfricansBouton pleine-largeur variante="secondaire" icone="fa-solid fa-share-nodes" @click="afficherModalPartage = true">
            Partager ce profil
          </AfricansBouton>
          <EngagementOffrirCadeauBouton
            type-objet="profil"
            :objet-id="id"
            :destinataire="`${profil.prenom} ${profil.nom}`"
            taille="sm"
            @offert="cadeauxRef?.rafraichir()"
          />
          <button
            type="button"
            :disabled="profilSignale"
            class="inline-flex h-10 w-full items-center justify-center gap-2 rounded-lg border text-base font-bold transition"
            :class="profilSignale
              ? 'cursor-default border-af-live/40 text-af-live'
              : 'border-af-bordure text-af-corps hover:border-af-live hover:text-af-live'"
            @click="afficherModalSignalement = true"
          >
            <font-awesome-icon icon="fa-solid fa-flag" />
            {{ profilSignale ? 'Profil signalé' : 'Signaler ce profil' }}
          </button>
        </div>
      </AfricansPanneau>

      <!--
        Cadeaux reçus : HORS du bloc « peutNoter » : un visiteur déconnecté doit
        voir la reconnaissance reçue par ce membre, même s'il ne peut pas
        lui-même offrir. Le composant se masque seul quand il n'y a rien.
      -->
      <AfricansPanneau>
        <EngagementCadeauxRecus ref="cadeauxRef" type-objet="profil" :objet-id="id" />
      </AfricansPanneau>
      </template>
    </template>

    <SocialRendezVousProposerModal
      v-if="afficherModalRdv && membreLight"
      :membre="membreLight"
      @fermer="afficherModalRdv = false"
      @propose="afficherModalRdv = false"
    />

    <ProfilPartagerProfilModal
      v-if="profil"
      ref="modalPartageRef"
      :is-open="afficherModalPartage"
      :profil-nom="profil.nom"
      :profil-prenom="profil.prenom"
      @close="afficherModalPartage = false"
      @submit="soumettrePartage"
    />

    <ProfilSignalerProfilModal
      v-if="profil"
      ref="modalSignalementRef"
      :is-open="afficherModalSignalement"
      :profil-nom="profil.nom"
      :profil-prenom="profil.prenom"
      @close="afficherModalSignalement = false"
      @submit="soumettreSignalement"
    />
  </NuxtLayout>
</template>
