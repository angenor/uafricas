<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="annonce?.titre ?? 'Annonce'"
        :sous-titre="annonce ? `${annonce.categorie}${annonce.ville ? ' · ' + annonce.ville : ''}` : undefined"
        :image="annonce?.photo_url ?? null"
      >
        <template v-if="annonce" #action>
          <span
            class="rounded-lg px-4 py-2 text-[14px]/[1.4] font-bold"
            :class="classeTypeEchange(annonce.type_echange)"
          >
            {{ annonce.type_echange }}
          </span>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Afromarket', vers: '/marche-africain' }, { libelle: annonce?.titre ?? 'Annonce' }]"
      />
    </template>

    <div v-if="loading" class="flex flex-col gap-5">
      <div v-for="n in 3" :key="n" class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="!annonce" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-store" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Annonce introuvable</p>
      <p class="mt-2 text-[14px]/[1.4] text-af-corps">Elle a peut-être été retirée par son auteur.</p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" vers="/marche-africain">
        Retour aux annonces
      </AfricansBouton>
    </div>

    <div v-else class="flex flex-col gap-5">
      <!-- Prix et métadonnées -->
      <div class="flex flex-col gap-4 rounded-[10px] border border-af-bordure bg-white p-6">
        <p class="text-[32px]/[1.2] font-bold text-af-chocolat">{{ prixFormate }}</p>

        <p class="flex flex-wrap items-center gap-x-5 gap-y-2 text-[12px]/[1.4] text-af-atone">
          <span class="flex items-center gap-1.5">
            <font-awesome-icon icon="fa-solid fa-location-dot" />
            {{ paysAffiche }}<template v-if="annonce.ville"> · {{ annonce.ville }}</template>
          </span>
          <span class="flex items-center gap-1.5">
            <font-awesome-icon icon="fa-solid fa-calendar-days" />
            Publié le {{ dateFormatee }}
          </span>
          <span class="flex items-center gap-1.5">
            <font-awesome-icon icon="fa-solid fa-tag" />
            {{ annonce.categorie }}
          </span>
          <span v-if="annonce.secteur" class="flex items-center gap-1.5">
            <font-awesome-icon icon="fa-solid fa-briefcase" />
            {{ annonce.secteur }}
          </span>
        </p>

        <AfricansEtiquette v-if="annonce.quantite && annonce.quantite > 1" class="self-start">
          <font-awesome-icon icon="fa-solid fa-boxes-stacked" class="mr-1.5" />
          Quantité minimum : {{ annonce.quantite }} unités
        </AfricansEtiquette>
      </div>

      <img
        v-if="annonce.photo_url"
        :src="annonce.photo_url"
        :alt="annonce.titre"
        class="w-full rounded-[10px] border border-af-bordure object-cover"
      />

      <AfricansAccordeon titre="Description" icone="fa-solid fa-align-left" fond="blanc" par-defaut-ouvert>
        <p class="text-[14px]/[1.4] whitespace-pre-line text-af-corps">{{ annonce.description }}</p>
      </AfricansAccordeon>

      <AfricansAccordeon
        v-if="annonce.type_annonceur === 'entreprise' && (annonce.nom_entreprise || annonce.contact_telephone || annonce.contact_email || annonce.contact_adresse)"
        titre="Coordonnées de l'annonceur"
        icone="fa-solid fa-building"
      >
        <ul class="flex flex-col gap-3">
          <li v-if="annonce.nom_entreprise" class="flex items-center gap-3 text-[14px]/[1.4] font-bold text-af-encre">
            <font-awesome-icon icon="fa-solid fa-building" class="w-4 text-af-atone" />
            {{ annonce.nom_entreprise }}
          </li>
          <li v-if="annonce.contact_telephone" class="flex items-center gap-3 text-[14px]/[1.4] text-af-corps">
            <font-awesome-icon icon="fa-solid fa-phone" class="w-4 text-af-atone" />
            <a :href="`tel:${annonce.contact_telephone}`" class="transition hover:text-af-chocolat">{{ annonce.contact_telephone }}</a>
          </li>
          <li v-if="annonce.contact_email" class="flex items-center gap-3 text-[14px]/[1.4] text-af-corps">
            <font-awesome-icon icon="fa-solid fa-envelope" class="w-4 text-af-atone" />
            <a :href="`mailto:${annonce.contact_email}`" class="transition hover:text-af-chocolat">{{ annonce.contact_email }}</a>
          </li>
          <li v-if="annonce.contact_adresse" class="flex items-center gap-3 text-[14px]/[1.4] text-af-corps">
            <font-awesome-icon icon="fa-solid fa-location-dot" class="w-4 text-af-atone" />
            {{ annonce.contact_adresse }}
          </li>
          <li v-if="annonce.site_web_url">
            <a
              :href="annonce.site_web_url"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center gap-3 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
            >
              <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" class="w-4" />
              Site web / réseau social
            </a>
          </li>
        </ul>
      </AfricansAccordeon>
    </div>

    <template #rail>
      <template v-if="annonce">
        <AfricansPanneau titre="Annonceur" icone="fa-solid fa-user">
          <div class="flex flex-col gap-4">
            <div class="flex items-center gap-3">
              <span class="grid size-12 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-[17px]/[1.4] font-bold text-af-chocolat">
                {{ annonce.user.prenom.charAt(0) }}{{ annonce.user.nom.charAt(0) }}
              </span>
              <p class="text-[14px]/[1.4] font-bold text-af-encre">
                {{ annonce.user.prenom }} {{ annonce.user.nom }}
              </p>
            </div>

            <!-- Chaque badge dit ce qui est vérifié ET ce qui ne l'est pas :
                 masquer les non-vérifiés laisserait croire à un profil complet. -->
            <ul class="flex flex-col gap-2">
              <li
                v-for="badge in badgesCredibilite"
                :key="badge.cle"
                class="flex items-center gap-2 text-[12px]/[1.4]"
                :class="badge.valide ? 'text-af-vert' : 'text-af-atone'"
                :title="badge.valide ? badge.libelleValide : badge.libelleInvalide"
              >
                <font-awesome-icon :icon="badge.valide ? 'fa-solid fa-circle-check' : 'fa-solid fa-circle-xmark'" />
                {{ badge.libelle }}
              </li>
            </ul>
          </div>
        </AfricansPanneau>

        <AfricansPanneau titre="Actions" icone="fa-solid fa-hand-point-up">
          <div v-if="estProprietaire" class="flex flex-col gap-3">
            <p class="text-[14px]/[1.4] text-af-corps">Ceci est votre annonce.</p>
            <AfricansBouton pleine-largeur icone="fa-solid fa-sliders" vers="/marche-africain/mes-annonces">
              Gérer mes annonces
            </AfricansBouton>
          </div>

          <div v-else-if="isAuthenticated" class="flex flex-col gap-3">
            <AfricansBouton pleine-largeur icone="fa-solid fa-envelope" @click="ouvrirContact">
              Contacter l'auteur
            </AfricansBouton>
            <MarcheFavoriBouton :annonce-id="annonce.id" avec-libelle variante="detail" class="w-full" />
          </div>

          <div v-else class="flex flex-col gap-3">
            <p class="text-[14px]/[1.4] text-af-corps">
              Connectez-vous pour contacter l'auteur de cette annonce.
            </p>
            <AfricansBouton pleine-largeur icone="fa-solid fa-right-to-bracket" :vers="`/login?redirect=/marche-africain/${annonce.id}`">
              Se connecter
            </AfricansBouton>
          </div>
        </AfricansPanneau>
      </template>
    </template>

    <!-- Contact : la conversation s'ouvre ensuite dans la messagerie ancrée. -->
    <AfricansModale
      v-model="showContactModal"
      titre="Contacter l'annonceur"
      :sous-titre="annonce?.titre"
    >
      <div class="flex flex-col gap-4">
        <AfricansChamp
          v-model="messageContact"
          libelle="Votre message"
          type="textarea"
          placeholder="Bonjour, je suis intéressé(e) par votre annonce…"
          obligatoire
        />
        <p v-if="erreurContact" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5" />
          {{ erreurContact }}
        </p>
      </div>

      <template #actions>
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70"
          @click="showContactModal = false"
        >
          Annuler
        </button>
        <AfricansBouton
          :desactive="contactEnCours"
          :tourne="contactEnCours"
          :icone="contactEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
          @click="envoyerContact"
        >
          {{ contactEnCours ? 'Envoi…' : 'Envoyer' }}
        </AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import {
  useMarcheAfricain,
  formatPrix,
  formatDate,
  classeTypeEchange,
  type AnnonceDetailAPI,
} from '~/composables/useMarcheAfricain'
import { useMessagerie } from '~/composables/useMessagerie'
import { useUserStore } from '~/stores/user'
import { navigateTo } from '#app'

definePageMeta({ layout: false })

const route = useRoute()
const userStore = useUserStore()
const { obtenirAnnonce, contacterAuteur, erreur } = useMarcheAfricain()
const { listerConversations, demanderOuverture } = useMessagerie()
const { redirigerVersConnexion } = useAuth()

// State
const loading = ref(true)
const annonce = ref<AnnonceDetailAPI | null>(null)
const showContactModal = ref(false)
const messageContact = ref('')
const contactEnCours = ref(false)
const erreurContact = ref<string | null>(null)

// Computed
const isAuthenticated = computed(() => userStore.isAuthenticated)
const estProprietaire = computed(
  () => isAuthenticated.value && annonce.value?.user.uid === userStore.user?.id,
)

const prixFormate = computed(() => {
  if (!annonce.value) return ''
  return formatPrix(annonce.value.prix, annonce.value.devise)
})

const dateFormatee = computed(() => {
  if (!annonce.value) return ''
  return formatDate(annonce.value.created_at)
})

const breadcrumbs = computed(() => [
  { label: 'Marché Africain', to: '/marche-africain' },
  { label: annonce.value?.titre || 'Détail', to: undefined },
])

// Badges de credibilite de l'annonceur (vert = verifie, rouge = non verifie)
const badgesCredibilite = computed(() => {
  const u = annonce.value?.user
  return [
    {
      cle: 'telephone',
      icone: 'phone',
      libelle: 'Téléphone',
      valide: !!u?.telephone_verifie,
      libelleValide: 'Numéro de téléphone validé par OTP',
      libelleInvalide: 'Numéro de téléphone non validé',
    },
    {
      cle: 'identite',
      icone: 'id-card',
      libelle: 'Identité',
      valide: !!u?.documents_verifie,
      libelleValide: "Pièce d'identité vérifiée",
      libelleInvalide: "Pièce d'identité non vérifiée",
    },
    {
      cle: 'compte',
      icone: 'user-shield',
      libelle: 'Compte validé',
      valide: !!u?.compte_valide,
      libelleValide: "Compte validé par l'administration",
      libelleInvalide: "Compte non validé par l'administration",
    },
  ]
})

const paysAffiche = computed(() => {
  if (!annonce.value) return ''
  if (annonce.value.pays.length > 0) {
    return annonce.value.pays.join(', ')
  }
  return 'Non spécifié'
})

const ouvrirContact = () => {
  if (!isAuthenticated.value) {
    redirigerVersConnexion()
    return
  }
  erreurContact.value = null
  showContactModal.value = true
}

const envoyerContact = async () => {
  if (!annonce.value) return
  if (messageContact.value.trim().length === 0) {
    erreurContact.value = 'Veuillez écrire un message.'
    return
  }
  contactEnCours.value = true
  erreurContact.value = null
  try {
    const resultat = await contacterAuteur(annonce.value.id, messageContact.value.trim())
    if (resultat) {
      showContactModal.value = false
      messageContact.value = ''
      // Ouvre la fenêtre de messagerie sur la conversation avec l'auteur
      await listerConversations()
      demanderOuverture({
        id: resultat.ami_id,
        nom: annonce.value.user.nom,
        prenom: annonce.value.user.prenom,
        slug: null,
        photoUrl: null,
        fonction: null,
        pays: null,
      })
    } else {
      erreurContact.value = erreur.value || "Impossible de contacter l'auteur."
    }
  } finally {
    contactEnCours.value = false
  }
}

// Lifecycle
onMounted(async () => {
  const id = route.params.id as string
  const resultat = await obtenirAnnonce(id)
  annonce.value = resultat

  if (annonce.value) {
    useHead({
      title: `${annonce.value.titre} - Marché Africain - AfricanS`,
      meta: [
        {
          name: 'description',
          content: annonce.value.description.substring(0, 160),
        },
      ],
    })
  }

  loading.value = false
})
</script>
