<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Soumettre un projet"
        sous-titre="Présentez votre projet de développement et bénéficiez du soutien de notre communauté d'investisseurs et de partenaires africains"
        image="/images/finance_projet_banire.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Financer un projet', vers: '/financer-projet' },
          { libelle: 'Soumettre un projet' },
        ]"
      >
        <template #centre>
          <p v-if="!succes" class="text-base font-bold text-af-encre">
            Étape {{ etape + 1 }} sur {{ ETAPES.length }}
          </p>
        </template>
      </AfricansFilAriane>
    </template>

    <!-- ═══ Confirmation ═══
         Elle REMPLACE le formulaire au lieu de le coiffer : laisser dix-huit
         champs vidés sous un message de succès se lit comme un formulaire à
         remplir de nouveau. -->
    <div v-if="succes" class="rounded-[10px] border border-af-vert/30 bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-5xl text-af-vert" />
      <h1 class="mt-4 text-[24px]/[1.3] font-bold text-af-encre">Projet soumis</h1>
      <p class="mx-auto mt-2 max-w-lg text-[14px]/[1.5] text-af-corps">
        Il sera examiné par notre équipe avant publication. Vous serez prévenu de la décision.
      </p>
      <div class="mt-6 flex flex-wrap justify-center gap-3">
        <AfricansBouton icone="fa-solid fa-magnifying-glass" vers="/financer-projet">
          Voir les projets
        </AfricansBouton>
        <AfricansBouton variante="secondaire" icone="fa-solid fa-plus" @click="recommencer">
          Soumettre un autre projet
        </AfricansBouton>
      </div>
    </div>

    <form v-else class="flex flex-col gap-6" @submit.prevent="surSoumission">
      <!-- ═══ Fil des étapes ═══
           Les pastilles sont des BOUTONS : revenir corriger une réponse deux
           étapes plus haut ne doit pas obliger à ressortir de celles qu'on a
           déjà remplies. Elles restent bloquées tant que l'étape 1 n'est pas
           valide, puisque c'est la seule qui porte des champs obligatoires. -->
      <nav class="flex flex-wrap gap-2" aria-label="Étapes du formulaire">
        <button
          v-for="(e, i) in ETAPES"
          :key="e.titre"
          type="button"
          class="flex min-w-0 flex-1 items-center gap-3 rounded-[10px] border p-3 text-left transition disabled:cursor-not-allowed disabled:opacity-50"
          :class="i === etape
            ? 'border-af-chocolat bg-af-chocolat/[0.07]'
            : 'border-af-bordure bg-white hover:border-af-chocolat'"
          :disabled="i > 0 && !etapeUnValide"
          :aria-current="i === etape ? 'step' : undefined"
          @click="allerA(i)"
        >
          <span
            class="grid size-8 shrink-0 place-items-center rounded-full text-[14px]/[1] font-bold"
            :class="i < etape
              ? 'bg-af-vert text-white'
              : i === etape
                ? 'bg-af-chocolat text-white'
                : 'bg-af-fond text-af-atone'"
          >
            <font-awesome-icon v-if="i < etape" icon="fa-solid fa-check" />
            <template v-else>{{ i + 1 }}</template>
          </span>
          <span class="min-w-0">
            <span class="block truncate text-[14px]/[1.3] font-bold" :class="i === etape ? 'text-af-chocolat' : 'text-af-encre'">
              {{ e.titre }}
            </span>
            <span class="block truncate text-[12px]/[1.3] text-af-atone">{{ e.obligatoire ? 'Obligatoire' : 'Facultatif' }}</span>
          </span>
        </button>
      </nav>

      <!-- Restauration. Le message dit AUSSI ce qui n'a pas été retrouvé :
           découvrir l'absence de l'image au moment d'envoyer serait pire que
           de l'apprendre tout de suite. -->
      <div
        v-if="brouillonRestaure"
        class="flex flex-wrap items-center gap-3 rounded-[10px] border border-af-vert/30 bg-af-vert/5 px-4 py-3 text-[14px]/[1.4] text-af-corps"
      >
        <font-awesome-icon icon="fa-solid fa-clock-rotate-left" class="shrink-0 text-af-vert" />
        <span class="min-w-0 flex-1">
          Votre saisie précédente a été retrouvée. L'image de couverture, elle, est à choisir de nouveau.
        </span>
        <button
          type="button"
          class="shrink-0 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
          @click="abandonnerBrouillon"
        >
          Repartir de zéro
        </button>
        <button
          type="button"
          class="grid size-6 shrink-0 place-items-center text-af-atone transition hover:text-af-encre"
          aria-label="Masquer ce message"
          @click="brouillonRestaure = false"
        >
          <font-awesome-icon icon="fa-solid fa-xmark" />
        </button>
      </div>

      <div v-if="erreurMessage" class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreurMessage }}
      </div>

      <!-- `v-show` et non `v-if` : démonter une étape recrée son champ de
           fichier au retour, et le navigateur perd alors le nom du fichier
           choisi — alors que la sélection, elle, vit dans `couvertureFile`. -->
      <section class="flex flex-col gap-5 rounded-[10px] border border-af-bordure bg-white p-6">
        <!-- ─── 1. Le projet ─── -->
        <template v-if="etape === 0">
          <header>
            <h2 class="text-[17px]/[1.4] font-bold">Le projet</h2>
            <p class="mt-1 text-[14px]/[1.4] text-af-corps">
              Les trois champs de cette étape sont les seuls exigés. Tout le reste peut être complété plus tard.
            </p>
          </header>

          <AfricansChamp
            v-model="form.titre"
            libelle="Titre du projet *"
            placeholder="Ex : construction d'un centre de formation numérique"
          />

          <AfricansChamp
            v-model="form.description"
            libelle="Description du projet *"
            type="textarea"
            placeholder="Contexte, besoins identifiés, approche proposée…"
          />

          <div class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Objectifs du projet *</span>
            <div v-for="(_, index) in objectifs" :key="index" class="flex items-center gap-2">
              <span class="w-6 shrink-0 text-right text-[14px]/[1.4] text-af-atone">{{ index + 1 }}.</span>
              <input
                v-model="objectifs[index]"
                type="text"
                :placeholder="`Objectif ${index + 1}`"
                class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
              />
              <button
                v-if="objectifs.length > 1"
                type="button"
                class="grid size-9 shrink-0 place-items-center rounded-md text-af-atone transition hover:text-af-live"
                :aria-label="`Retirer l'objectif ${index + 1}`"
                @click="supprimerObjectif(index)"
              >
                <font-awesome-icon icon="fa-solid fa-trash" />
              </button>
            </div>
            <button
              type="button"
              class="flex w-fit items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
              @click="ajouterObjectif"
            >
              <font-awesome-icon icon="fa-solid fa-plus" />
              Ajouter un objectif
            </button>
          </div>

          <div class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Image de couverture</span>
            <div v-if="couverturePreview" class="relative w-fit">
              <img :src="couverturePreview" alt="Aperçu de la couverture" class="h-32 w-48 rounded-[10px] border border-af-bordure object-cover" />
              <button
                type="button"
                class="absolute -top-2 -right-2 grid size-6 place-items-center rounded-full bg-af-live text-xs text-white transition hover:opacity-80"
                aria-label="Retirer l'image"
                @click="supprimerCouverture"
              >
                <font-awesome-icon icon="fa-solid fa-xmark" />
              </button>
            </div>
            <input
              id="couverture"
              type="file"
              accept="image/*"
              class="text-[14px]/[1.4] text-af-corps file:mr-4 file:rounded-md file:border-0 file:bg-af-chocolat file:px-4 file:py-2 file:text-[14px]/[1.4] file:font-bold file:text-white hover:file:opacity-90"
              @change="handleCouvertureChange"
            />
            <p class="text-[12px]/[1.4] text-af-atone">5 Mo maximum.</p>
          </div>
        </template>

        <!-- ─── 2. Le porteur ─── -->
        <template v-else-if="etape === 1">
          <header>
            <h2 class="text-[17px]/[1.4] font-bold">Le porteur</h2>
            <p class="mt-1 text-[14px]/[1.4] text-af-corps">
              Qui porte le projet, où, et comment vous joindre.
            </p>
          </header>

          <AfricansChamp v-model="form.nomOrganisation" libelle="Nom de l'organisation" placeholder="Ex : ONG Développement Durable Afrique" />
          <AfricansChamp v-model="form.descriptionOrganisation" libelle="Description de l'organisation" type="textarea" placeholder="Présentez brièvement votre organisation…" />
          <AfricansChamp v-model="form.siteWeb" libelle="Site web" type="url" placeholder="https://www.exemple.org" />

          <div class="grid gap-5 sm:grid-cols-2">
            <AfricansChamp v-model="form.pays" libelle="Territoire" type="select">
              <option value="">Choisir un territoire</option>
              <option v-for="pays in paysOptions" :key="pays.value" :value="pays.value">{{ pays.label }}</option>
            </AfricansChamp>
            <AfricansChamp v-model="form.ville" libelle="Ville" placeholder="Ex : Dakar" />
          </div>

          <div class="grid gap-5 sm:grid-cols-2">
            <AfricansChamp v-model="form.contactEmail" libelle="Courriel de contact" type="email" placeholder="contact@exemple.org" />
            <AfricansChamp v-model="form.contactTelephone" libelle="Téléphone" type="tel" placeholder="+221 77 000 00 00" />
          </div>
        </template>

        <!-- ─── 3. Budget et calendrier ─── -->
        <template v-else-if="etape === 2">
          <header>
            <h2 class="text-[17px]/[1.4] font-bold">Budget et calendrier</h2>
            <p class="mt-1 text-[14px]/[1.4] text-af-corps">
              Ce que le projet coûte, et quand il se déroule.
            </p>
          </header>

          <div class="grid gap-5 sm:grid-cols-2">
            <!-- Champs natifs et non `AfricansChamp` : le composant n'émet que
                 des chaînes, et le coût comme la durée partent à l'API en
                 nombres. `12` deviendrait `"12"`, refusé à la désérialisation. -->
            <label class="flex flex-col gap-2">
              <span class="text-[14px]/[1.4] text-af-atone italic">Coût total estimé</span>
              <input
                v-model.number="form.coutTotal"
                type="number"
                min="0"
                step="1000"
                placeholder="Ex : 5000000"
                class="h-11 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
              />
            </label>

            <AfricansChamp v-model="form.devise" libelle="Devise" type="select">
              <option v-for="devise in DEVISES_FORM" :key="devise.value" :value="devise.value">{{ devise.label }}</option>
            </AfricansChamp>

            <label class="flex flex-col gap-2">
              <span class="text-[14px]/[1.4] text-af-atone italic">Durée du projet</span>
              <select
                v-model="form.dureeMois"
                class="h-11 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
              >
                <option :value="null">Choisir une durée</option>
                <option v-for="duree in DUREES_MOIS_FORM" :key="duree.value" :value="duree.value">{{ duree.label }}</option>
              </select>
            </label>

            <label class="flex flex-col gap-2">
              <span class="text-[14px]/[1.4] text-af-atone italic">Date de début souhaitée</span>
              <input
                v-model="form.dateDebutSouhaitee"
                type="date"
                class="h-11 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
              />
            </label>
          </div>
        </template>

        <!-- ─── 4. Le détail ─── -->
        <template v-else>
          <header>
            <h2 class="text-[17px]/[1.4] font-bold">Le détail</h2>
            <p class="mt-1 text-[14px]/[1.4] text-af-corps">
              Ce que vous en attendez, comment vous comptez y arriver, et ce qui pourrait l'en empêcher.
            </p>
          </header>

          <AfricansChamp v-model="form.resultatsAttendus" libelle="Résultats attendus" type="textarea" placeholder="Quels changements concrets ce projet produira-t-il ?" />
          <AfricansChamp v-model="form.activitesProgrammees" libelle="Activités programmées" type="textarea" placeholder="Les grandes activités prévues." />
          <AfricansChamp v-model="form.echeanciers" libelle="Échéanciers" type="textarea" placeholder="Les jalons et leurs dates." />
          <AfricansChamp v-model="form.contributionAutonomisation" libelle="Contribution à l'autonomisation" type="textarea" placeholder="En quoi le projet rend-il les bénéficiaires plus autonomes ?" />
          <AfricansChamp v-model="form.difficultesRisques" libelle="Difficultés et risques" type="textarea" placeholder="Ce qui pourrait compromettre le projet, et comment vous l'anticipez." />
        </template>
      </section>

      <!-- ═══ Navigation ═══ -->
      <div class="flex flex-wrap items-center gap-3">
        <AfricansBouton v-if="etape > 0" variante="secondaire" icone="fa-solid fa-arrow-left" @click="precedent">
          Précédent
        </AfricansBouton>

        <AfricansBouton
          v-if="etape < ETAPES.length - 1"
          icone="fa-solid fa-arrow-right"
          :desactive="etape === 0 && !etapeUnValide"
          @click="suivant"
        >
          Suivant
        </AfricansBouton>

        <!-- Sortie anticipée. Quinze des dix-huit champs sont facultatifs :
             obliger à traverser trois étapes vides pour envoyer un dossier
             déjà complet serait une formalité, pas un accompagnement. -->
        <AfricansBouton
          v-if="etape < ETAPES.length - 1 && isFormValid"
          variante="secondaire"
          icone="fa-solid fa-paper-plane"
          :desactive="loading"
          :tourne="loading"
          @click="surSoumission"
        >
          Soumettre maintenant
        </AfricansBouton>

        <AfricansBouton
          v-if="etape === ETAPES.length - 1"
          type="submit"
          :icone="loading ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
          :desactive="!isFormValid || loading"
          :tourne="loading"
        >
          {{ loading ? 'Soumission en cours…' : 'Soumettre le projet' }}
        </AfricansBouton>

        <AfricansBouton class="ml-auto" variante="secondaire" vers="/financer-projet">
          Annuler
        </AfricansBouton>
      </div>
    </form>

    <template v-if="!succes" #rail>
      <AfricansPanneau titre="Progression" icone="fa-solid fa-list-check">
        <ol class="flex flex-col gap-1">
          <li v-for="(e, i) in ETAPES" :key="e.titre">
            <button
              type="button"
              class="flex w-full items-start gap-3 rounded-lg px-3 py-2.5 text-left transition disabled:cursor-not-allowed disabled:opacity-50"
              :class="i === etape ? 'bg-af-chocolat/15' : 'hover:bg-af-chocolat/[0.07]'"
              :disabled="i > 0 && !etapeUnValide"
              @click="allerA(i)"
            >
              <span
                class="mt-0.5 grid size-6 shrink-0 place-items-center rounded-full text-[12px]/[1] font-bold"
                :class="i < etape ? 'bg-af-vert text-white' : i === etape ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-atone'"
              >
                <font-awesome-icon v-if="i < etape" icon="fa-solid fa-check" />
                <template v-else>{{ i + 1 }}</template>
              </span>
              <span class="min-w-0">
                <span class="block text-[14px]/[1.3] font-bold" :class="i === etape ? 'text-af-chocolat' : 'text-af-encre'">{{ e.titre }}</span>
                <span class="block text-[12px]/[1.4] text-af-atone">{{ e.resume }}</span>
              </span>
            </button>
          </li>
        </ol>
      </AfricansPanneau>

      <AfricansPanneau titre="Bon à savoir" icone="fa-solid fa-circle-info">
        <ul class="flex flex-col gap-3 text-[14px]/[1.5] text-af-corps">
          <li class="flex gap-3">
            <font-awesome-icon icon="fa-solid fa-asterisk" class="mt-1 size-3 shrink-0 text-af-chocolat" />
            Seuls le titre, la description et un objectif sont exigés. Vous pouvez envoyer dès qu'ils sont remplis.
          </li>
          <li class="flex gap-3">
            <font-awesome-icon icon="fa-solid fa-eye" class="mt-1 size-3 shrink-0 text-af-chocolat" />
            Votre projet est examiné par l'équipe avant d'être publié.
          </li>
          <li class="flex gap-3">
            <font-awesome-icon icon="fa-solid fa-floppy-disk" class="mt-1 size-3 shrink-0 text-af-vert" />
            Votre saisie est conservée sur cet appareil si vous quittez la page. L'image de couverture fait exception.
          </li>
        </ul>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
import {
  useProjets,
  PAYS_PROJETS,
  DEVISES_FORM,
  DUREES_MOIS_FORM,
} from '~/composables/useProjets'
import { useUserStore } from '~/stores/user'

/**
 * Soumettre un projet, porté sur le gabarit et découpé en QUATRE ÉTAPES.
 *
 * Le formulaire tenait sur une seule page : dix-huit champs, dont cinq zones
 * de texte long, sur près de quatre écrans de défilement. On n'y voyait ni où
 * l'on en était, ni ce qu'il restait.
 *
 * Le découpage suit une DISSYMÉTRIE du formulaire : seuls trois champs sont
 * exigés (titre, description, un objectif), les quinze autres sont
 * facultatifs. L'étape 1 les rassemble donc tous les trois, et le bouton
 * « Soumettre maintenant » apparaît dès qu'elle est valide — traverser trois
 * étapes vides pour envoyer un dossier déjà complet serait une formalité, pas
 * un accompagnement.
 *
 * Aucun champ n'est ajouté ni retiré, et la charge utile envoyée à
 * `creerProjet` est rigoureusement la même.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Soumettre un projet | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Soumettez votre projet de développement africain et bénéficiez du soutien de notre communauté.',
    },
  ],
})

const router = useRouter()
const userStore = useUserStore()
const { creerProjet, chargement } = useProjets()
const { redirigerVersConnexion } = useAuth()

// Auth guard
onMounted(() => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
  }
})

// ─── Étapes ───────────────────────────────────────────────────────────────
const ETAPES = [
  { titre: 'Le projet', resume: 'Titre, description, objectifs, image', obligatoire: true },
  { titre: 'Le porteur', resume: 'Organisation, lieu, contact', obligatoire: false },
  { titre: 'Budget et calendrier', resume: 'Coût, devise, durée, date', obligatoire: false },
  { titre: 'Le détail', resume: 'Résultats, activités, risques', obligatoire: false },
] as const

const etape = ref(0)

/** Seule l'étape 1 porte des champs exigés : c'est elle qui garde les autres. */
const etapeUnValide = computed(() =>
  form.titre.trim().length > 0
  && form.description.trim().length > 0
  && objectifs.some(o => o.trim().length > 0),
)

const allerA = (i: number) => {
  if (i > 0 && !etapeUnValide.value) return
  etape.value = i
  // Remonter en tête : sans cela, changer d'étape laisse le visiteur au bas
  // de la précédente, devant des champs qu'il n'a pas encore vus.
  if (import.meta.client) window.scrollTo({ top: 0, behavior: 'smooth' })
}

const suivant = () => allerA(etape.value + 1)
const precedent = () => allerA(etape.value - 1)

// Options pays (sans l'option "Tous les pays")
const paysOptions = PAYS_PROJETS.filter(p => p.value !== '')

// Form state
/**
 * Valeurs par défaut, déclarées UNE fois. Elles servaient à trois endroits :
 * l'état initial, la remise à zéro et, désormais, le test « ce brouillon
 * vaut-il d'être conservé ». Trois listes de dix-huit champs recopiées à la
 * main auraient fini par diverger d'un champ, en silence.
 */
const valeursInitiales = () => ({
  titre: '',
  description: '',
  nomOrganisation: '',
  descriptionOrganisation: '',
  siteWeb: '',
  pays: '',
  ville: '',
  coutTotal: null as number | null,
  devise: 'XOF',
  dureeMois: null as number | null,
  dateDebutSouhaitee: '',
  resultatsAttendus: '',
  activitesProgrammees: '',
  echeanciers: '',
  contributionAutonomisation: '',
  difficultesRisques: '',
  contactEmail: '',
  contactTelephone: '',
})

const form = reactive(valeursInitiales())

const objectifs = reactive<string[]>([''])
const couvertureFile = ref<File | null>(null)
const couverturePreview = ref<string | null>(null)
const loading = computed(() => chargement.value)
const succes = ref(false)
const erreurMessage = ref<string | null>(null)

// La validité du formulaire EST celle de l'étape 1 : elle seule porte des
// champs exigés. Une seconde définition aurait pu diverger de la garde des
// étapes sans que rien ne le signale.
const isFormValid = etapeUnValide

// Objectifs
const ajouterObjectif = () => {
  objectifs.push('')
}

const supprimerObjectif = (index: number) => {
  objectifs.splice(index, 1)
}

// Couverture
const handleCouvertureChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (!target.files || !target.files[0]) return

  const file = target.files[0]

  // Vérifier la taille (5 Mo max)
  if (file.size > 5 * 1024 * 1024) {
    erreurMessage.value = 'L\'image ne doit pas dépasser 5 Mo.'
    target.value = ''
    return
  }

  couvertureFile.value = file

  // Preview
  const reader = new FileReader()
  reader.onload = (e) => {
    couverturePreview.value = e.target?.result as string
  }
  reader.readAsDataURL(file)
}

const supprimerCouverture = () => {
  couvertureFile.value = null
  couverturePreview.value = null
  // Reset file input
  const input = document.getElementById('couverture') as HTMLInputElement
  if (input) input.value = ''
}

// Reset
const resetForm = () => {
  Object.assign(form, valeursInitiales())
  objectifs.splice(0, objectifs.length, '')
  supprimerCouverture()
}

// ─── Brouillon local ──────────────────────────────────────────────────────
//
// Un formulaire en quatre étapes fait passer plus de temps dessus, donc plus
// à perdre : un rechargement, un lien cliqué par erreur, et dix-huit champs
// disparaissaient.
//
// La clé porte l'identifiant du membre. Sur un poste partagé, une clé unique
// ferait relire à quelqu'un le brouillon d'un autre — et le lui ferait
// soumettre sous son propre compte.
//
// Elle porte aussi un numéro de version : si la forme du formulaire change,
// un brouillon écrit par l'ancienne version ne doit pas être réinjecté
// champ par champ dans la nouvelle.
const CLE_BROUILLON = computed(
  () => `africans:brouillon:projet:v1:${userStore.user?.id ?? 'anonyme'}`,
)

/** Vrai le temps d'annoncer au membre que sa saisie a été retrouvée. */
const brouillonRestaure = ref(false)

/**
 * Un brouillon vide n'est pas un brouillon. Sans ce test, ouvrir la page puis
 * la quitter écrirait dix-huit champs vides dans le stockage, et le bandeau
 * « saisie retrouvée » s'afficherait à la visite suivante sans rien à montrer.
 *
 * La comparaison porte sur les valeurs par défaut RÉELLES, pas sur « chaîne
 * vide ou nul » : `devise` vaut `XOF` d'emblée, et la traiter comme une saisie
 * rendrait tout brouillon utile.
 */
const brouillonUtile = () => {
  const defauts = valeursInitiales() as Record<string, unknown>
  return Object.entries(form).some(([cle, valeur]) => valeur !== defauts[cle])
    || objectifs.some(o => o.trim() !== '')
}

const enregistrerBrouillon = () => {
  if (!import.meta.client || succes.value) return
  try {
    if (!brouillonUtile()) {
      localStorage.removeItem(CLE_BROUILLON.value)
      return
    }
    // La couverture est ABSENTE du brouillon : c'est un `File`, que JSON ne
    // sait pas porter, et son aperçu en base64 pèse un tiers de plus que
    // l'image — 5 Mo autorisés deviendraient 6,7 Mo, au-delà du quota de
    // `localStorage`. Le membre en est prévenu à la restauration.
    localStorage.setItem(CLE_BROUILLON.value, JSON.stringify({
      form: { ...form },
      objectifs: [...objectifs],
      etape: etape.value,
    }))
  }
  catch {
    // Quota dépassé, ou stockage refusé par le navigateur. Le formulaire
    // continue de fonctionner sans filet plutôt que de s'interrompre.
  }
}

const effacerBrouillon = () => {
  if (!import.meta.client) return
  try {
    localStorage.removeItem(CLE_BROUILLON.value)
  }
  catch { /* stockage indisponible */ }
}

const restaurerBrouillon = () => {
  if (!import.meta.client) return
  let brut: string | null = null
  try {
    brut = localStorage.getItem(CLE_BROUILLON.value)
  }
  catch { return }
  if (!brut) return

  try {
    const donnees = JSON.parse(brut) as {
      form?: Record<string, unknown>
      objectifs?: unknown
      etape?: unknown
    }
    // Recopie champ par champ, et seulement ceux que le formulaire connaît :
    // un brouillon trafiqué ne doit pas pouvoir injecter de clé étrangère
    // dans l'objet envoyé à l'API.
    for (const cle of Object.keys(form) as (keyof typeof form)[]) {
      const valeur = donnees.form?.[cle]
      if (valeur !== undefined) (form as Record<string, unknown>)[cle] = valeur
    }
    if (Array.isArray(donnees.objectifs) && donnees.objectifs.length) {
      objectifs.splice(0, objectifs.length, ...donnees.objectifs.map(o => String(o)))
    }
    if (typeof donnees.etape === 'number' && donnees.etape >= 0 && donnees.etape < ETAPES.length) {
      etape.value = donnees.etape
    }
    brouillonRestaure.value = true
  }
  catch {
    // Brouillon illisible : on le jette plutôt que de laisser un formulaire
    // à moitié rempli d'on ne sait quoi.
    effacerBrouillon()
  }
}

/** Repart d'un formulaire vierge et oublie le brouillon. */
const abandonnerBrouillon = () => {
  resetForm()
  effacerBrouillon()
  brouillonRestaure.value = false
  etape.value = 0
}

// L'enregistrement est différé : sans cela, chaque frappe écrirait dans
// `localStorage`, une opération SYNCHRONE qui bloque le fil principal.
let minuterieBrouillon: ReturnType<typeof setTimeout> | null = null
watch([form, objectifs, etape], () => {
  if (minuterieBrouillon) clearTimeout(minuterieBrouillon)
  minuterieBrouillon = setTimeout(enregistrerBrouillon, 500)
}, { deep: true })

// Fermer l'onglet ne laisse pas le temps à la minuterie de se déclencher.
const surFermeture = () => {
  if (minuterieBrouillon) clearTimeout(minuterieBrouillon)
  enregistrerBrouillon()
}

onMounted(() => {
  restaurerBrouillon()
  window.addEventListener('beforeunload', surFermeture)
})
onBeforeUnmount(() => {
  window.removeEventListener('beforeunload', surFermeture)
  surFermeture()
})

/** Relance une saisie après une confirmation. */
const recommencer = () => {
  succes.value = false
  erreurMessage.value = null
  etape.value = 0
}

// Submit
const surSoumission = async () => {
  if (!isFormValid.value) return

  erreurMessage.value = null
  succes.value = false

  try {
    // Construire les objectifs en JSON
    const objectifsFiltres = objectifs.filter(o => o.trim().length > 0)
    const objectifsJson = JSON.stringify(objectifsFiltres)

    const result = await creerProjet(
      {
        titre: form.titre.trim(),
        description: form.description.trim(),
        objectifs: objectifsJson,
        nom_organisation: form.nomOrganisation || undefined,
        description_organisation: form.descriptionOrganisation || undefined,
        site_web: form.siteWeb || undefined,
        pays: form.pays || undefined,
        ville: form.ville || undefined,
        contact_email: form.contactEmail || undefined,
        contact_telephone: form.contactTelephone || undefined,
        cout_total: form.coutTotal ?? undefined,
        devise: form.devise || undefined,
        duree_mois: form.dureeMois ?? undefined,
        date_commencement_souhaitee: form.dateDebutSouhaitee || undefined,
        resultats_attendus: form.resultatsAttendus || undefined,
        activites_programmees: form.activitesProgrammees || undefined,
        echeanciers: form.echeanciers || undefined,
        contribution_autonomisation: form.contributionAutonomisation || undefined,
        difficultes_risques: form.difficultesRisques || undefined,
      },
      couvertureFile.value,
    )

    if (result) {
      succes.value = true
      effacerBrouillon()
      brouillonRestaure.value = false
      resetForm()
      etape.value = 0
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } else {
      erreurMessage.value = 'Une erreur est survenue lors de la soumission du projet.'
    }
  } catch (e: any) {
    erreurMessage.value = e?.message || 'Une erreur est survenue lors de la soumission.'
  }
}
</script>
